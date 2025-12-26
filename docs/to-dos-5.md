# anyform Phase 5: Survey & Quiz Polish

> **Status**: Pending
> **PRD Reference**: [axum-sea-forms-prd-0.4.0.md](/Users/rick/p/wordpuppi/docs/prd/libs/asf/axum-sea-forms-prd-0.4.0.md) Section 13
> **Depends On**: Phase 4.x (complete)

---

## Overview

Phase 5 completes the quiz/survey functionality. The database schema and entities are ready, but the **scoring engine is missing** - submissions currently save with `score: None`.

**What exists:**
- `af_results` table with `min_score`, `max_score`, `key`, `title`, `description`
- `submission.score`, `submission.max_score`, `submission.result_key` fields
- `field.correct_answer`, `field.points`, `field.weight` metadata
- `option.is_correct`, `option.points` metadata
- `form_settings.is_quiz`, `form_settings.show_answers` flags
- `Result::find_by_score()` query method

**What's missing:**
- Scoring calculation logic
- Handler integration to score quiz submissions
- Result bucket CRUD API
- Analytics helpers

---

## 1. Scoring Engine

### 1.1 Create QuizScorer Service
**File:** `anyform/src/services/scoring.rs`

- [ ] Create `QuizScorer` struct
- [ ] Implement `calculate_score()` method:
  ```rust
  pub async fn calculate_score(
      db: &DatabaseConnection,
      form: &form::Model,
      submission_data: &HashMap<String, FieldValue>,
  ) -> Result<ScoreResult, Error>
  ```
- [ ] Return `ScoreResult { score: i32, max_score: i32, result_key: Option<String> }`

### 1.2 Scoring Logic
- [ ] Iterate over submitted field values
- [ ] Match against `field.correct_answer` for single-value fields
- [ ] Match against `option.is_correct` for select/radio/checkbox fields
- [ ] Award `option.points` or `field.points` for correct answers
- [ ] Calculate `max_score` from all scorable fields
- [ ] Apply `field.weight` if configured (multiply points by weight)

### 1.3 Partial Credit (Multi-Select)
- [ ] For checkbox fields with multiple correct options:
  - Award points per correct selection
  - Optionally deduct for incorrect selections (configurable)
- [ ] Handle "all or nothing" vs "partial credit" modes

### 1.4 Score Normalization
- [ ] Implement `score_percentage()` helper: `(score / max_score) * 100`
- [ ] Support normalized scoring (0-100 scale)

### 1.5 Result Bucket Matching
- [ ] After calculating score, call `Result::find_by_score(db, form_id, score)`
- [ ] Set `result_key` from matched bucket
- [ ] Return `None` if no bucket matches

---

## 2. Handler Integration

### 2.1 Update submit_form Handler
**File:** `anyform/src/handlers/mod.rs`

- [ ] After validation passes, check `form.settings().is_quiz`
- [ ] If quiz, call `QuizScorer::calculate_score()`
- [ ] Update submission ActiveModel with score fields:
  ```rust
  score: Set(Some(result.score)),
  max_score: Set(Some(result.max_score)),
  result_key: Set(result.result_key),
  ```
- [ ] Save updated submission

### 2.2 Update SubmissionResponse
**File:** `anyform/src/handlers/responses.rs`

- [ ] Ensure `score`, `max_score`, `result_key` are exposed in API response
- [ ] Add `result` field with full result bucket data (title, description)
- [ ] Add `score_percentage` computed field

---

## 3. Result Bucket Management API

### 3.1 Admin Endpoints
**File:** `anyform/src/handlers/admin.rs`

- [ ] `GET /api/admin/forms/{form_id}/results` - List result buckets
- [ ] `POST /api/admin/forms/{form_id}/results` - Create result bucket
- [ ] `GET /api/admin/forms/{form_id}/results/{result_id}` - Get result bucket
- [ ] `PUT /api/admin/forms/{form_id}/results/{result_id}` - Update result bucket
- [ ] `DELETE /api/admin/forms/{form_id}/results/{result_id}` - Delete result bucket

### 3.2 Input Types
**File:** `anyform/src/services/form_builder.rs`

- [ ] Create `CreateResultInput` struct:
  ```rust
  pub struct CreateResultInput {
      pub key: String,
      pub title: String,
      pub description: Option<String>,
      pub min_score: i32,
      pub max_score: i32,
      pub order: i32,
  }
  ```
- [ ] Create `UpdateResultInput` struct
- [ ] Add `FormBuilder::create_result()` method
- [ ] Add `FormBuilder::update_result()` method
- [ ] Add `FormBuilder::delete_result()` method

### 3.3 CLI Commands
**File:** `anyform/src/bin/anyform.rs` or `anyform/src/commands/`

- [ ] `anyform result list <form-slug>` - List result buckets
- [ ] `anyform result create <form-slug> --key <key> --title <title> --min <score> --max <score>`
- [ ] `anyform result delete <form-slug> <result-key>`

---

## 4. Analytics Helpers

### 4.1 Create Analytics Service
**File:** `anyform/src/services/analytics.rs`

- [ ] Create module with helper functions
- [ ] Export from `services/mod.rs`

### 4.2 NPS Calculation
- [ ] Implement `calculate_nps()` for NPS-type fields (0-10 rating):
  ```rust
  pub fn calculate_nps(ratings: &[i32]) -> NpsResult {
      // Promoters: 9-10
      // Passives: 7-8
      // Detractors: 0-6
      // NPS = %Promoters - %Detractors
  }
  ```
- [ ] Return `NpsResult { score: f64, promoters: usize, passives: usize, detractors: usize }`

### 4.3 Rating Average
- [ ] Implement `calculate_average()` for rating fields:
  ```rust
  pub fn calculate_average(values: &[f64]) -> f64
  ```
- [ ] Support weighted averages

### 4.4 Response Distribution
- [ ] Implement `calculate_distribution()` for select/radio fields:
  ```rust
  pub fn calculate_distribution(responses: &[String]) -> HashMap<String, DistributionEntry>
  ```
- [ ] Return count and percentage per option

### 4.5 Analytics Export
- [ ] `GET /api/admin/forms/{form_id}/analytics` endpoint
- [ ] Return JSON with:
  - Submission count
  - Average score (for quizzes)
  - Score distribution
  - Field-level analytics (ratings, distributions)

---

## 5. Tests

### 5.1 Scoring Tests
**File:** `anyform/src/services/scoring.rs` (inline tests)

- [ ] Test single correct answer scoring
- [ ] Test multiple choice scoring
- [ ] Test checkbox partial credit
- [ ] Test weighted scoring
- [ ] Test result bucket matching
- [ ] Test edge cases (no correct answer, zero points, etc.)

### 5.2 Integration Tests
**File:** `anyform/tests/quiz_tests.rs`

- [ ] Test complete quiz submission workflow
- [ ] Test score appears in submission response
- [ ] Test result bucket selection
- [ ] Test analytics calculations

---

## 6. Documentation

- [ ] Update README with quiz/survey features
- [ ] Add example quiz form JSON
- [ ] Document scoring algorithm
- [ ] Document analytics API

---

## Summary

| Task | Priority | Effort |
|------|----------|--------|
| Create QuizScorer service | High | Medium |
| Update submit_form handler | High | Low |
| Result bucket CRUD API | Medium | Medium |
| CLI result commands | Low | Low |
| Analytics helpers | Medium | Medium |
| Tests | High | Medium |
| Documentation | Low | Low |

**Estimated total effort**: 1-2 days

---

## File Reference

| File | Action |
|------|--------|
| `anyform/src/services/mod.rs` | Add `scoring` and `analytics` modules |
| `anyform/src/services/scoring.rs` | **Create** - QuizScorer |
| `anyform/src/services/analytics.rs` | **Create** - Analytics helpers |
| `anyform/src/handlers/mod.rs` | **Update** - Call scorer in submit_form |
| `anyform/src/handlers/admin.rs` | **Update** - Add result CRUD endpoints |
| `anyform/src/handlers/responses.rs` | **Update** - Add result to submission response |
| `anyform/src/services/form_builder.rs` | **Update** - Add result CRUD methods |
| `anyform/src/bin/anyform.rs` | **Update** - Add result CLI commands |
| `anyform/tests/quiz_tests.rs` | **Create** - Integration tests |

---

*Created: 2025-12-26*
