use log::warn;
use serde::{Deserialize, Deserializer, Serialize};

pub type Courses = Vec<Course>;

pub enum Course{
    EmptyCourse{
        id: i64,
    },
    FullCourse(FullCourse)
}
impl<'de> Deserialize<'de> for Course{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        if let Ok(course) = FullCourse::deserialize(deserializer){
            Ok(Course::FullCourse(course))
        }else{
            warn!("TODO: Implement EmptyCourse");
            Ok(Course::EmptyCourse{
                id: 0,
            })
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct FullCourse {
    id: i64,
    name: String,
    account_id: Option<i64>,
    uuid: Option<String>,
    start_at: Option<String>,
    grading_standard_id: Option<serde_json::Value>,
    is_public: Option<bool>,
    created_at: Option<String>,
    course_code: Option<String>,
    default_view: Option<String>,
    root_account_id: Option<i64>,
    enrollment_term_id: Option<i64>,
    license: Option<String>,
    grade_passback_setting: Option<serde_json::Value>,
    end_at: Option<String>,
    public_syllabus: Option<bool>,
    public_syllabus_to_auth: Option<bool>,
    storage_quota_mb: Option<i64>,
    is_public_to_auth_users: Option<bool>,
    homeroom_course: Option<bool>,
    course_color: Option<serde_json::Value>,
    friendly_name: Option<serde_json::Value>,
    apply_assignment_group_weights: Option<bool>,
    calendar: Option<Calendar>,
    time_zone: Option<String>,
    blueprint: Option<bool>,
    template: Option<bool>,
    enrollments: Option<Vec<Enrollment>>,
    hide_final_grades: Option<bool>,
    workflow_state: Option<String>,
    course_format: Option<String>,
    restrict_enrollments_to_course_dates: Option<bool>,
    overridden_course_visibility: Option<String>,
    access_restricted_by_date: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Calendar {
    ics: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Enrollment {
    #[serde(rename = "type")]
    enrollment_type: String,
    role: String,
    role_id: i64,
    user_id: i64,
    enrollment_state: String,
    limit_privileges_to_course_section: bool,
}
#[cfg(test)]
pub mod tests {
    use crate::courses::{Course, Courses};
    use crate::test::get_url;
    use crate::{test, Pagination};
    use reqwest::Client;
    use std::str::FromStr;
    async fn get_courses(client: &Client, url: &str) -> (Courses, Pagination) {
        let response = client.get(url).send().await.expect("Failed to get courses");
        if !response.status().is_success() {
            panic!("Failed to load courses: {}", response.status());
        }

        let option = response
            .headers()
            .get("link")
            .expect("Failed to get link header");
        let pagination = Pagination::from_str(
            option
                .to_str()
                .expect("Failed to convert link header to str"),
        )
        .expect("Failed to parse link header");
        let response = response
            .json::<Courses>()
            .await
            .expect("Failed to parse response");
        (response, pagination)
    }
    #[tokio::test]
    pub async fn load_courses() {
        let client = test::build_client();
        let (mut courses, mut pagination) = get_courses(&client, &get_url("courses")).await;
        while let Some(next) = pagination.next {
            let (mut next_courses, next_pagination) = get_courses(&client, &next).await;
            courses.append(&mut next_courses);
            pagination = next_pagination;
        }
        for course in courses {
            match course {
                Course::EmptyCourse { .. } => {}
                Course::FullCourse(ok) => {
                    println!("{:?}", ok.name);
                }
            }
        }
    }
}
