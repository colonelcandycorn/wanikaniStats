use super::*;

impl CompleteUserInfo {
    pub fn get_user_name(&self) -> &str {
        &self.user.username
    }

    pub fn get_level(&self) -> i32 {
        self.user.level
    }

    pub fn get_num_of_resets(&self) -> i32 {
        self.resets.len() as i32
    }

    pub fn get_date_of_most_recent_reset(&self) -> Option<&DateTime<Local>> {
        self.resets.iter().map(|reset| &reset.confirmed_at).max()
    }

    pub fn get_started_at(&self) -> &DateTime<Local> {
        &self.user.started_at
    }

    pub fn get_kanji_learned(&self) -> i32 {
        self.kanji_learned
    }

    pub fn get_radicals_learned(&self) -> i32 {
        self.radicals_learned
    }

    pub fn get_vocab_learned(&self) -> i32 {
        self.kana_learned + self.vocab_learned
    }

    pub fn get_total_correct_reading_count(&self) -> i32 {
        self.kanji_stats.num_of_reading_correct
            + self.vocab_stats.num_of_reading_correct
            + self.kana_stats.num_of_reading_correct
    }

    pub fn get_total_correct_meaning_count(&self) -> i32 {
        self.kanji_stats.num_of_meaning_correct
            + self.vocab_stats.num_of_meaning_correct
            + self.kana_stats.num_of_meaning_correct
            + self.radical_stats.num_of_meaning_correct
    }

    pub fn get_total_incorrect_reading_count(&self) -> i32 {
        self.kanji_stats.num_of_reading_incorrect
            + self.vocab_stats.num_of_reading_incorrect
            + self.kana_stats.num_of_reading_incorrect
    }

    pub fn get_total_incorrect_meaning_count(&self) -> i32 {
        self.kanji_stats.num_of_meaning_incorrect
            + self.vocab_stats.num_of_meaning_incorrect
            + self.kana_stats.num_of_meaning_incorrect
            + self.radical_stats.num_of_meaning_incorrect
    }

    pub fn get_total_reading_count(&self) -> i32 {
        self.get_total_correct_reading_count() + self.get_total_incorrect_reading_count()
    }

    pub fn get_total_meaning_count(&self) -> i32 {
        self.get_total_correct_meaning_count() + self.get_total_incorrect_meaning_count()
    }

    pub fn get_total_correct_count(&self) -> i32 {
        self.get_total_correct_meaning_count() + self.get_total_correct_reading_count()
    }

    pub fn get_total_incorrect_count(&self) -> i32 {
        self.get_total_incorrect_meaning_count() + self.get_total_incorrect_reading_count()
    }

    pub fn get_total_count(&self) -> i32 {
        self.get_total_reading_count() + self.get_total_meaning_count()
    }

    pub fn get_total_accuracy(&self) -> f64 {
        let total_correct = (self.get_total_correct_meaning_count()
            + self.get_total_correct_reading_count()) as f64;

        (total_correct / self.get_total_count() as f64) * 100.0
    }

    pub fn get_total_reading_accuracy(&self) -> f64 {
        (self.get_total_correct_reading_count() as f64 / self.get_total_reading_count() as f64)
            * 100.0
    }

    pub fn get_total_meaning_accuracy(&self) -> f64 {
        (self.get_total_correct_meaning_count() as f64 / self.get_total_meaning_count() as f64)
            * 100.0
    }

    pub fn get_radical_meaning_accuracy(&self) -> f64 {
        let radical_count = (self.radical_stats.num_of_meaning_correct
            + self.radical_stats.num_of_meaning_incorrect) as f64;

        (self.radical_stats.num_of_meaning_correct as f64 / radical_count) * 100.0
    }

    pub fn get_kanji_reading_accuracy(&self) -> f64 {
        let kanji_count = (self.kanji_stats.num_of_reading_correct
            + self.kanji_stats.num_of_reading_incorrect) as f64;

        (self.kanji_stats.num_of_reading_correct as f64 / kanji_count) * 100.0
    }

    pub fn get_kanji_meaning_accuracy(&self) -> f64 {
        let kanji_count = (self.kanji_stats.num_of_meaning_correct
            + self.kanji_stats.num_of_meaning_incorrect) as f64;

        (self.kanji_stats.num_of_meaning_correct as f64 / kanji_count) * 100.0
    }

    pub fn get_vocab_reading_accuracy(&self) -> f64 {
        let vocab_count = (self.vocab_stats.num_of_reading_correct
            + self.vocab_stats.num_of_reading_incorrect
            + self.kana_stats.num_of_reading_correct
            + self.kana_stats.num_of_reading_incorrect) as f64;
        let vocab_correct = (self.vocab_stats.num_of_reading_correct
            + self.kana_stats.num_of_reading_correct) as f64;

        (vocab_correct / vocab_count) * 100.0
    }

    pub fn get_vocab_meaning_accuracy(&self) -> f64 {
        let vocab_count = (self.vocab_stats.num_of_meaning_correct
            + self.vocab_stats.num_of_meaning_incorrect
            + self.kana_stats.num_of_meaning_correct
            + self.kana_stats.num_of_meaning_incorrect) as f64;
        let vocab_correct = (self.vocab_stats.num_of_meaning_correct
            + self.kana_stats.num_of_meaning_correct) as f64;

        (vocab_correct / vocab_count) * 100.0
    }

    pub fn get_kanji_total_accuracy(&self) -> f64 {
        let kanji_count = (self.kanji_stats.num_of_meaning_correct
            + self.kanji_stats.num_of_meaning_incorrect
            + self.kanji_stats.num_of_reading_correct
            + self.kanji_stats.num_of_reading_incorrect) as f64;
        let kanji_correct = (self.kanji_stats.num_of_meaning_correct
            + self.kanji_stats.num_of_reading_correct) as f64;

        (kanji_correct / kanji_count) * 100.0
    }

    pub fn get_vocab_total_accuracy(&self) -> f64 {
        let vocab_count = (self.vocab_stats.num_of_meaning_correct
            + self.vocab_stats.num_of_meaning_incorrect
            + self.vocab_stats.num_of_reading_correct
            + self.vocab_stats.num_of_reading_incorrect
            + self.kana_stats.num_of_meaning_correct
            + self.kana_stats.num_of_meaning_incorrect
            + self.kana_stats.num_of_reading_correct
            + self.kana_stats.num_of_reading_incorrect) as f64;
        let vocab_correct = (self.vocab_stats.num_of_meaning_correct
            + self.vocab_stats.num_of_reading_correct
            + self.kana_stats.num_of_meaning_correct
            + self.kana_stats.num_of_reading_correct) as f64;

        (vocab_correct / vocab_count) * 100.0
    }
}

impl CompleteUserInfoBuilder {
    /// Creates a new `CompleteUserInfoBuilder`. Realistically, I think it would be better
    /// for this to have no parameters and instead have methods to add the data. This would
    /// allow for more flexibility in the future. I mostly did this because I was trying to
    /// find the easiest way to add in the stats and other calculated fields.
    ///
    /// I also don't like that this is public because it is only used in the `ApiClient` struct, but
    /// because that struct is in a different module, I had to make this public.
    pub fn new(
        user: User,
        review_stats: Vec<ReviewStatistic>,
        assignments: Vec<Assignment>,
        resets: Vec<Reset>,
        id_to_subjects: HashMap<i32, SubjectWithType>,
    ) -> CompleteUserInfoBuilder {
        CompleteUserInfoBuilder {
            user,
            review_stats,
            assignments,
            resets,
            id_to_subjects,
        }
    }

    /// This method mostly just exists to do all the calculations and return the `CompleteUserInfo`
    /// struct. So realistically, you just call new, add all the data, and then call this method.
    pub fn build(self) -> Result<CompleteUserInfo, MissingSubjectError> {
        let kanji_stats = self.get_subject_type_stats(&SubjectType::Kanji).unwrap();
        let vocab_stats = self
            .get_subject_type_stats(&SubjectType::Vocabulary)
            .unwrap();
        let kana_stats = self
            .get_subject_type_stats(&SubjectType::KanaVocabulary)
            .unwrap();
        let radical_stats = self.get_subject_type_stats(&SubjectType::Radical).unwrap();
        let kanji_learned = self.get_num_of_passed(SubjectType::Kanji).unwrap();
        let radicals_learned = self.get_num_of_passed(SubjectType::Radical).unwrap();
        let vocab_learned = self.get_num_of_passed(SubjectType::Vocabulary).unwrap();
        let kana_learned = self.get_num_of_passed(SubjectType::KanaVocabulary).unwrap();

        Ok(CompleteUserInfo {
            user: self.user,
            review_stats: self.review_stats,
            assignments: self.assignments,
            resets: self.resets,
            id_to_subjects: self.id_to_subjects,
            kana_stats,
            kanji_stats,
            radical_stats,
            vocab_stats,
            kana_learned,
            kanji_learned,
            radicals_learned,
            vocab_learned,
        })
    }

    fn get_num_of_passed(&self, subject: SubjectType) -> Result<i32, MissingSubjectError> {
        let mut result = 0;

        for assignment in &self.assignments {
            let subject_obj = self.id_to_subjects.get(&assignment.subject_id);

            if subject_obj.is_none() {
                return Err(MissingSubjectError);
            }

            let subject_type = &subject_obj.unwrap().subject_type;

            if *subject_type == subject && assignment.passed_at.is_some() {
                result += 1;
            }
        }

        Ok(result)
    }

    fn get_subject_type_stats(
        &self,
        subject: &SubjectType,
    ) -> Result<SubjectTypeStats, MissingSubjectError> {
        let mut meaning_correct = 0;
        let mut meaning_incorrect = 0;
        let mut reading_correct = 0;
        let mut reading_incorrect = 0;

        for review_stat in &self.review_stats {
            let subject_obj = self.id_to_subjects.get(&review_stat.subject_id);

            if subject_obj.is_none() {
                return Err(MissingSubjectError);
            }

            let subject_type = &subject_obj.unwrap().subject_type;

            if subject_type == subject {
                meaning_correct += review_stat.meaning_correct;
                meaning_incorrect += review_stat.meaning_incorrect;
                reading_correct += review_stat.reading_correct;
                reading_incorrect += review_stat.reading_incorrect;
            }
        }

        Ok(SubjectTypeStats {
            subject_type: subject.clone(),
            num_of_meaning_correct: meaning_correct,
            num_of_meaning_incorrect: meaning_incorrect,
            num_of_reading_correct: reading_correct,
            num_of_reading_incorrect: reading_incorrect,
        })
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use chrono::{Local, TimeZone};

    /// Generates a static fake `ReviewStatistic`
    pub fn fake_review_statistic(subject_id: i32, subject_type: &str) -> ReviewStatistic {
        ReviewStatistic {
            created_at: Local.with_ymd_and_hms(2023, 10, 1, 12, 0, 0).unwrap(),
            meaning_correct: 50,
            meaning_current_streak: 5,
            meaning_incorrect: 10,
            meaning_max_streak: 10,
            percentage_correct: 80,
            reading_correct: 40,
            reading_current_streak: 3,
            reading_incorrect: 5,
            reading_max_streak: 8,
            subject_id,
            subject_type: subject_type.to_string(),
        }
    }

    /// Generates a static fake `Assignment`
    pub fn fake_assignment(subject_id: i32) -> Assignment {
        Assignment {
            created_at: Some(Local.with_ymd_and_hms(2023, 10, 1, 12, 0, 0).unwrap()),
            passed_at: Some(Local.with_ymd_and_hms(2023, 10, 2, 12, 0, 0).unwrap()),
            srs_stage: 5,
            subject_id,
        }
    }

    pub fn fake_non_passed_assignment(subject_id: i32) -> Assignment {
        Assignment {
            created_at: Some(Local.with_ymd_and_hms(2023, 10, 1, 12, 0, 0).unwrap()),
            passed_at: None,
            srs_stage: 5,
            subject_id,
        }
    }

    /// Generates a static fake `Subject`
    pub fn fake_subject(subject_type: &str) -> Subject {
        Subject {
            characters: match subject_type {
                "radical" => Some("一".to_string()),
                "kanji" => Some("日".to_string()),
                "vocabulary" => Some("食べる".to_string()),
                "kana_vocabulary" => Some("たべる".to_string()),
                _ => None,
            },
            level: 5,
            spaced_repetition_system_id: 1,
            meanings: vec![
                Meanings {
                    meaning: Some("one".to_string()),
                    primary: true,
                },
                Meanings {
                    meaning: Some("first".to_string()),
                    primary: false,
                },
            ],
        }
    }

    /// Generates a static fake `Reset`
    pub fn fake_reset() -> Reset {
        Reset {
            created_at: Local.with_ymd_and_hms(2023, 10, 1, 12, 0, 0).unwrap(),
            confirmed_at: Local.with_ymd_and_hms(2023, 10, 2, 12, 0, 0).unwrap(),
            original_level: 10,
            target_level: 5,
        }
    }

    fn setup_builder() -> CompleteUserInfoBuilder {
        let user = User {
            username: "test".to_string(),
            level: 1,
            started_at: Local::now(),
        };
        let review_stats = vec![fake_review_statistic(1, "kanji")];
        let assignments = vec![fake_assignment(1)];
        let resets = vec![fake_reset()];
        let mut id_to_subjects = HashMap::new();

        id_to_subjects.insert(
            1,
            SubjectWithType::new(fake_subject("kanji"), SubjectType::Kanji),
        );

        CompleteUserInfoBuilder::new(user, review_stats, assignments, resets, id_to_subjects)
    }

    #[test]
    fn test_get_num_of_passed() {
        let builder = setup_builder();
        let subject = SubjectType::Kanji;

        let num_of_passed_kanji = builder.get_num_of_passed(subject).unwrap();

        assert_eq!(num_of_passed_kanji, 1);
    }

    #[test]
    fn test_non_passed_subject_type() {
        let mut builder = setup_builder();

        let non_passed_assignment = fake_non_passed_assignment(2);
        builder.assignments.push(non_passed_assignment);
        builder.id_to_subjects.insert(
            2,
            SubjectWithType::new(fake_subject("kanji"), SubjectType::Kanji),
        );

        let num_of_passed = builder.get_num_of_passed(SubjectType::Kanji).unwrap();

        assert!(builder.assignments.len() > 1);
        assert_eq!(num_of_passed, 1);
    }

    #[test]
    fn test_different_subject_type_does_not_affect_count() {
        let mut builder = setup_builder();

        let passed_assignment = fake_assignment(2);
        builder.assignments.push(passed_assignment);
        builder.id_to_subjects.insert(
            2,
            SubjectWithType::new(fake_subject("radical"), SubjectType::Radical),
        );

        let num_of_passed = builder.get_num_of_passed(SubjectType::Kanji).unwrap();
        let num_of_passed_radical = builder.get_num_of_passed(SubjectType::Radical).unwrap();

        assert!(builder.assignments.len() > 1);
        assert_eq!(num_of_passed, 1);
        assert_eq!(num_of_passed_radical, 1);
    }

    #[test]
    fn test_get_subject_type_stats() {
        let builder = setup_builder();
        let subject = SubjectType::Kanji;

        let stats = builder.get_subject_type_stats(&subject).unwrap();

        assert_eq!(stats.subject_type, subject);
        assert_eq!(stats.num_of_meaning_correct, 50);
        assert_eq!(stats.num_of_meaning_incorrect, 10);
        assert_eq!(stats.num_of_reading_correct, 40);
        assert_eq!(stats.num_of_reading_incorrect, 5);
    }

    #[test]
    fn test_insert_review_statistic_and_get_subject_type_stats() {
        let mut builder = setup_builder();
        let subject = SubjectType::Kanji;

        let review_stat = fake_review_statistic(2, "kanji");
        builder.review_stats.push(review_stat);
        builder.id_to_subjects.insert(
            2,
            SubjectWithType::new(fake_subject("kanji"), SubjectType::Kanji),
        );

        let stats = builder.get_subject_type_stats(&subject).unwrap();

        assert_eq!(stats.subject_type, subject);
        assert_eq!(stats.num_of_meaning_correct, 100);
        assert_eq!(stats.num_of_meaning_incorrect, 20);
        assert_eq!(stats.num_of_reading_correct, 80);
        assert_eq!(stats.num_of_reading_incorrect, 10);
    }

    #[test]
    fn test_get_basic_info() {
        let builder = setup_builder();
        let user_info = builder.build().unwrap();

        assert_eq!(user_info.get_user_name(), "test");
        assert_eq!(user_info.get_level(), 1);
        assert_eq!(user_info.get_num_of_resets(), 1);
        assert_eq!(user_info.get_kanji_learned(), 1);
        assert_eq!(user_info.get_radicals_learned(), 0);
        assert_eq!(user_info.get_vocab_learned(), 0);
        assert_eq!(
            user_info.get_date_of_most_recent_reset(),
            Some(&Local.with_ymd_and_hms(2023, 10, 2, 12, 0, 0).unwrap())
        );
    }
}
