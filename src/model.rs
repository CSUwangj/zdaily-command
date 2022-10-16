use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct QuestionMetadata {
  pub title: String,
  #[serde(rename(deserialize = "titleSlug"))]
  pub title_slug: String
}

#[derive(Deserialize)]
pub(crate) struct ActiveDailyQuestion {
  pub link: String,
  pub question: QuestionMetadata
}

#[derive(Deserialize)]
pub(crate) struct ActiveDailyQuestionWrapper {
  #[serde(rename(deserialize = "activeDailyCodingChallengeQuestion"))]
  pub active_daily_question: ActiveDailyQuestion
}

#[derive(Deserialize)]
pub(crate) struct DailyData {
  pub data: ActiveDailyQuestionWrapper
}

#[derive(Deserialize)]
pub(crate) struct QuestionDetail {
  pub content: String
}

#[derive(Deserialize)]
pub(crate) struct QuestionDetailWrapper {
  pub question: QuestionDetail
}

#[derive(Deserialize)]
pub(crate) struct QuestionData {
  pub data: QuestionDetailWrapper
}