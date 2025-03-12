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
        &self.kana_learned + &self.vocab_learned
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

    pub fn build(self) -> Result<CompleteUserInfo, MissingSubjectError> {
        let kanji_stats = self.get_subject_type_stats(SubjectType::Kanji).unwrap();
        let vocab_stats = self
            .get_subject_type_stats(SubjectType::Vocabulary)
            .unwrap();
        let kana_stats = self
            .get_subject_type_stats(SubjectType::KanaVocabulary)
            .unwrap();
        let radical_stats = self.get_subject_type_stats(SubjectType::Radical).unwrap();
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

    pub fn get_num_of_passed(&self, subject: SubjectType) -> Result<i32, MissingSubjectError> {
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

    pub fn get_subject_type_stats(
        &self,
        subject: SubjectType,
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

            if *subject_type == subject {
                meaning_correct += review_stat.meaning_correct;
                meaning_incorrect += review_stat.meaning_incorrect;
                reading_correct += review_stat.reading_correct;
                reading_incorrect += review_stat.reading_incorrect;
            }
        }

        Ok(SubjectTypeStats {
            subject_type: subject,
            num_of_meaning_correct: meaning_correct,
            num_of_meaning_incorrect: meaning_incorrect,
            num_of_reading_correct: reading_correct,
            num_of_reading_incorrect: reading_incorrect,
        })
    }
}
