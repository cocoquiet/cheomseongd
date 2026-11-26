use std::time::Duration;

use chrono::NaiveDate;
use reqwest::{Client, Url};
use scraper::{Html, Selector};

use crate::crawler::{CrawlError, CrawledNotice, Crawler};

const SOURCE_ID: &str = "computer";
const LIST_URL: &str = "https://computer.knu.ac.kr/bbs/board.php?bo_table=sub6_1_a";
const ORGANIZATION_NAME: &str = "경북대학교 컴퓨터학부";

/// Crawler for the KNU School of Computer Science and Engineering notice board.
///
/// The board is sorted newest-first, so each run reads its first page. The list
/// page contains the metadata needed by `CrawledNotice`; body text is not
/// available there and is left empty until detail-page parsing is added.
pub struct ComputerCrawler {
    client: Client,
}

impl ComputerCrawler {
    pub fn new() -> Result<Self, CrawlError> {
        let client = Client::builder()
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            ))
            .timeout(Duration::from_secs(20))
            .build()
            .map_err(|error| CrawlError::new(SOURCE_ID, error.to_string()))?;

        Ok(Self { client })
    }

    pub fn with_client(client: Client) -> Self {
        Self { client }
    }
}

impl Crawler for ComputerCrawler {
    fn source_id(&self) -> &'static str {
        SOURCE_ID
    }

    async fn crawl(&self) -> Result<Vec<CrawledNotice>, CrawlError> {
        let response = self
            .client
            .get(LIST_URL)
            .send()
            .await
            .map_err(|error| CrawlError::new(SOURCE_ID, error.to_string()))?
            .error_for_status()
            .map_err(|error| CrawlError::new(SOURCE_ID, error.to_string()))?;

        let html = response
            .text()
            .await
            .map_err(|error| CrawlError::new(SOURCE_ID, error.to_string()))?;

        parse_notices(&html)
    }
}

fn parse_notices(html: &str) -> Result<Vec<CrawledNotice>, CrawlError> {
    let document = Html::parse_document(html);
    let row_selector = Selector::parse("#bo_list tbody tr").expect("valid row selector");
    let title_selector = Selector::parse(".bo_tit a").expect("valid title selector");
    let category_selector = Selector::parse(".bo_cate_link").expect("valid category selector");
    let author_selector = Selector::parse(".td_name").expect("valid author selector");
    let date_selector = Selector::parse(".td_datetime").expect("valid date selector");
    let number_selector = Selector::parse(".td_num2").expect("valid number selector");

    let mut notices = Vec::new();

    for row in document.select(&row_selector) {
        let Some(title_link) = row.select(&title_selector).next() else {
            // Skip non-notice rows such as an empty-board message.
            continue;
        };

        let href = title_link
            .value()
            .attr("href")
            .ok_or_else(|| CrawlError::new(SOURCE_ID, "notice title link is missing its href"))?;
        let page_url = Url::parse(LIST_URL)
            .expect("valid list URL")
            .join(href)
            .map_err(|error| CrawlError::new(SOURCE_ID, error.to_string()))?;
        let source_key = page_url
            .query_pairs()
            .find_map(|(key, value)| (key == "wr_id").then(|| value.into_owned()))
            .ok_or_else(|| CrawlError::new(SOURCE_ID, "notice URL is missing wr_id"))?;

        let title = normalize_text(title_link.text());
        if title.is_empty() {
            return Err(CrawlError::new(SOURCE_ID, "notice title is empty"));
        }

        let category = row
            .select(&category_selector)
            .next()
            .map(|element| normalize_text(element.text()))
            .filter(|value| !value.is_empty());
        let author = row
            .select(&author_selector)
            .next()
            .map(|element| normalize_text(element.text()))
            .filter(|value| !value.is_empty());
        let published_on = row.select(&date_selector).next().and_then(|element| {
            NaiveDate::parse_from_str(&normalize_text(element.text()), "%Y-%m-%d").ok()
        });
        let is_pinned = row.value().classes().any(|class| class == "bo_notice")
            || row
                .select(&number_selector)
                .next()
                .is_some_and(|element| normalize_text(element.text()).contains("공지"));

        notices.push(CrawledNotice {
            organization_name: ORGANIZATION_NAME.to_owned(),
            source_key: format!("sub6_1_a:{source_key}"),
            source_url: page_url.into(),
            title,
            author,
            category,
            body_text: None,
            is_pinned,
            published_on,
        });
    }

    if notices.is_empty() {
        return Err(CrawlError::new(
            SOURCE_ID,
            "no notice rows found; the board markup may have changed",
        ));
    }

    Ok(notices)
}

fn normalize_text<'a>(text: impl Iterator<Item = &'a str>) -> String {
    text.collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::{ComputerCrawler, ORGANIZATION_NAME, SOURCE_ID, parse_notices};
    use crate::crawler::Crawler;
    use chrono::NaiveDate;

    const LIST_HTML: &str = r#"
        <div id="bo_list">
          <table><tbody>
            <tr class="even">
              <td class="td_num2">7059</td>
              <td class="td_subject">
                <a class="bo_cate_link" href="?sca=%EC%9D%BC%EB%B0%98%EA%B3%B5%EC%A7%80">일반공지</a>
                <div class="bo_tit">
                  <a href="https://computer.knu.ac.kr/bbs/board.php?bo_table=sub6_1_a&amp;wr_id=29538">
                    2026 겨울 계절학기&nbsp;학점교류 신청
                  </a>
                </div>
              </td>
              <td class="td_name">이민승</td>
              <td class="td_datetime">2026-11-26</td>
            </tr>
            <tr class="bo_notice">
              <td class="td_num2">공지</td>
              <td class="td_subject">
                <div class="bo_tit">
                  <a href="board.php?bo_table=sub6_1_a&amp;wr_id=100">
                    중요 공지
                  </a>
                </div>
              </td>
              <td class="td_name"></td>
              <td class="td_datetime">날짜 미정</td>
            </tr>
          </tbody></table>
        </div>
    "#;

    #[test]
    fn parses_notice_fields_and_absolute_urls() {
        let notices = parse_notices(LIST_HTML).expect("list HTML should parse");

        assert_eq!(notices.len(), 2);
        let notice = &notices[0];
        assert_eq!(notice.organization_name, ORGANIZATION_NAME);
        assert_eq!(notice.source_key, "sub6_1_a:29538");
        assert_eq!(
            notice.source_url,
            "https://computer.knu.ac.kr/bbs/board.php?bo_table=sub6_1_a&wr_id=29538"
        );
        assert_eq!(notice.title, "2026 겨울 계절학기 학점교류 신청");
        assert_eq!(notice.author.as_deref(), Some("이민승"));
        assert_eq!(notice.category.as_deref(), Some("일반공지"));
        assert_eq!(notice.published_on, NaiveDate::from_ymd_opt(2026, 11, 26));
        assert!(!notice.is_pinned);
        assert_eq!(notice.body_text, None);
    }

    #[test]
    fn parses_pinned_notice_and_relative_urls() {
        let notices = parse_notices(LIST_HTML).expect("list HTML should parse");
        let notice = &notices[1];

        assert_eq!(notice.source_key, "sub6_1_a:100");
        assert_eq!(
            notice.source_url,
            "https://computer.knu.ac.kr/bbs/board.php?bo_table=sub6_1_a&wr_id=100"
        );
        assert_eq!(notice.title, "중요 공지");
        assert_eq!(notice.author, None);
        assert_eq!(notice.category, None);
        assert_eq!(notice.published_on, None);
        assert!(notice.is_pinned);
    }

    #[test]
    fn returns_error_when_no_notice_rows_are_found() {
        let error = parse_notices("<div id=\"bo_list\"><p>게시물이 없습니다.</p></div>")
            .expect_err("missing rows should return an error");

        assert_eq!(error.source_id(), SOURCE_ID);
        assert!(error.message().contains("no notice rows found"));
    }

    #[test]
    fn crawler_exposes_expected_source_id() {
        let crawler = ComputerCrawler::with_client(reqwest::Client::new());
        assert_eq!(crawler.source_id(), "computer");
    }
}
