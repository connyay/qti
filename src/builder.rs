use crate::error::Result;
use crate::types::{AcceptableAnswer, Assessment, Choice, Question, QuestionType};
use uuid::Uuid;
use xmltree::{Element, XMLNode};

/// Builder for QTI XML elements
pub struct QtiBuilder {
    /// Whether to include Canvas-specific extensions
    canvas_extensions: bool,
}

impl QtiBuilder {
    pub fn new() -> Self {
        Self {
            canvas_extensions: false,
        }
    }

    pub fn with_canvas_extensions(mut self) -> Self {
        self.canvas_extensions = true;
        self
    }

    /// Build the root questestinterop element
    pub fn build_questestinterop(&self, assessment: &Assessment) -> Result<Element> {
        let mut root = Element::new("questestinterop");

        // Set namespaces
        root.attributes.insert(
            "xmlns".to_string(),
            "http://www.imsglobal.org/xsd/ims_qtiasiv1p2".to_string(),
        );
        root.attributes.insert(
            "xmlns:xsi".to_string(),
            "http://www.w3.org/2001/XMLSchema-instance".to_string(),
        );
        root.attributes.insert(
            "xsi:schemaLocation".to_string(),
            "http://www.imsglobal.org/xsd/ims_qtiasiv1p2 http://www.imsglobal.org/xsd/ims_qtiasiv1p2p1.xsd".to_string()
        );

        // Build assessment element
        let assessment_elem = self.build_assessment(assessment)?;
        root.children.push(XMLNode::Element(assessment_elem));

        Ok(root)
    }

    /// Build assessment element
    fn build_assessment(&self, assessment: &Assessment) -> Result<Element> {
        let mut elem = Element::new("assessment");
        elem.attributes
            .insert("ident".to_string(), assessment.identifier.clone());
        elem.attributes
            .insert("title".to_string(), assessment.title.clone());

        // Add metadata
        if self.canvas_extensions {
            elem.children
                .push(XMLNode::Element(self.build_qtimetadata(assessment)?));
        }

        // Add section with all items
        let section = self.build_section(assessment)?;
        elem.children.push(XMLNode::Element(section));

        Ok(elem)
    }

    fn build_qtimetadata(&self, assessment: &Assessment) -> Result<Element> {
        let mut metadata = Element::new("qtimetadata");

        // Time limit
        if let Some(time_limit) = assessment.time_limit {
            metadata.children.push(XMLNode::Element(
                self.build_qtimetadatafield("time_limit", &time_limit.to_string()),
            ));
        }

        // Shuffle settings
        if assessment.metadata.shuffle_questions {
            metadata.children.push(XMLNode::Element(
                self.build_qtimetadatafield("shuffle_questions", "true"),
            ));
        }

        if assessment.metadata.shuffle_answers {
            metadata.children.push(XMLNode::Element(
                self.build_qtimetadatafield("shuffle_answers", "true"),
            ));
        }

        Ok(metadata)
    }

    fn build_qtimetadatafield(&self, label: &str, entry: &str) -> Element {
        let mut field = Element::new("qtimetadatafield");

        let mut label_elem = Element::new("fieldlabel");
        label_elem.children.push(XMLNode::Text(label.to_string()));
        field.children.push(XMLNode::Element(label_elem));

        let mut entry_elem = Element::new("fieldentry");
        entry_elem.children.push(XMLNode::Text(entry.to_string()));
        field.children.push(XMLNode::Element(entry_elem));

        field
    }

    fn build_section(&self, assessment: &Assessment) -> Result<Element> {
        let mut section = Element::new("section");
        section
            .attributes
            .insert("ident".to_string(), format!("section_{}", Uuid::new_v4()));
        section
            .attributes
            .insert("title".to_string(), "Main Section".to_string());

        // Add all questions as items
        for question in &assessment.questions {
            let item = self.build_item(question)?;
            section.children.push(XMLNode::Element(item));
        }

        Ok(section)
    }

    /// Build item element for a question
    fn build_item(&self, question: &Question) -> Result<Element> {
        let mut item = Element::new("item");
        item.attributes
            .insert("ident".to_string(), question.id.clone());
        item.attributes.insert(
            "title".to_string(),
            if question.title.is_empty() {
                format!("Question {}", &question.id[9..15])
            } else {
                question.title.clone()
            },
        );

        // Add item metadata if using Canvas extensions
        if self.canvas_extensions {
            item.children
                .push(XMLNode::Element(self.build_itemmetadata(question)?));
        }

        // Add presentation
        item.children
            .push(XMLNode::Element(self.build_presentation(question)?));

        // Add response processing
        item.children
            .push(XMLNode::Element(self.build_resprocessing(question)?));

        // Add feedback if present
        if let Some(ref feedback) = question.feedback {
            if let Some(ref correct_feedback) = feedback.correct {
                item.children.push(XMLNode::Element(
                    self.build_itemfeedback("correct", correct_feedback),
                ));
            }
            if let Some(ref incorrect_feedback) = feedback.incorrect {
                item.children.push(XMLNode::Element(
                    self.build_itemfeedback("incorrect", incorrect_feedback),
                ));
            }
        }

        Ok(item)
    }

    fn build_itemmetadata(&self, question: &Question) -> Result<Element> {
        let mut metadata = Element::new("itemmetadata");

        let mut field = Element::new("qtimetadatafield");

        let mut label = Element::new("fieldlabel");
        label
            .children
            .push(XMLNode::Text("question_type".to_string()));
        field.children.push(XMLNode::Element(label));

        let mut entry = Element::new("fieldentry");
        let qtype = match &question.question_type {
            QuestionType::MultipleChoice { .. } => "multiple_choice_question",
            QuestionType::TrueFalse { .. } => "true_false_question",
            QuestionType::MultipleAnswer { .. } => "multiple_answers_question",
            QuestionType::ShortAnswer { .. } => "short_answer_question",
            QuestionType::Numerical { .. } => "numerical_question",
            QuestionType::Essay { .. } => "essay_question",
            QuestionType::FileUpload { .. } => "file_upload_question",
        };
        entry.children.push(XMLNode::Text(qtype.to_string()));
        field.children.push(XMLNode::Element(entry));

        metadata.children.push(XMLNode::Element(field));

        // Points
        let mut points_field = Element::new("qtimetadatafield");
        let mut points_label = Element::new("fieldlabel");
        points_label
            .children
            .push(XMLNode::Text("points_possible".to_string()));
        points_field.children.push(XMLNode::Element(points_label));
        let mut points_entry = Element::new("fieldentry");
        points_entry
            .children
            .push(XMLNode::Text(question.points.to_string()));
        points_field.children.push(XMLNode::Element(points_entry));
        metadata.children.push(XMLNode::Element(points_field));

        Ok(metadata)
    }

    fn build_presentation(&self, question: &Question) -> Result<Element> {
        let mut presentation = Element::new("presentation");

        // Add question text as material
        let mut material = Element::new("material");
        let mut mattext = Element::new("mattext");
        mattext
            .attributes
            .insert("texttype".to_string(), "text/html".to_string());
        mattext.children.push(XMLNode::Text(question.text.clone()));
        material.children.push(XMLNode::Element(mattext));
        presentation.children.push(XMLNode::Element(material));

        // Add response based on question type
        match &question.question_type {
            QuestionType::MultipleChoice { choices, shuffle } => {
                let response = self.build_response_lid(question, choices, *shuffle)?;
                presentation.children.push(XMLNode::Element(response));
            }
            QuestionType::MultipleAnswer { choices, .. } => {
                let response = self.build_response_lid(question, choices, false)?;
                presentation.children.push(XMLNode::Element(response));
            }
            QuestionType::TrueFalse { .. } => {
                // True/False is a special case of multiple choice
                let choices = vec![Choice::new("True", false), Choice::new("False", false)];
                let response = self.build_response_lid(question, &choices, false)?;
                presentation.children.push(XMLNode::Element(response));
            }
            QuestionType::ShortAnswer { .. } => {
                let response = self.build_response_str(question)?;
                presentation.children.push(XMLNode::Element(response));
            }
            QuestionType::Numerical { .. } => {
                let response = self.build_response_num(question)?;
                presentation.children.push(XMLNode::Element(response));
            }
            QuestionType::Essay { .. } => {
                let response = self.build_response_str(question)?;
                presentation.children.push(XMLNode::Element(response));
            }
            QuestionType::FileUpload { .. } => {
                // File upload uses a special response type
                let response = self.build_response_str(question)?;
                presentation.children.push(XMLNode::Element(response));
            }
        }

        Ok(presentation)
    }

    fn build_response_lid(
        &self,
        question: &Question,
        choices: &[Choice],
        shuffle: bool,
    ) -> Result<Element> {
        let mut response = Element::new("response_lid");
        response
            .attributes
            .insert("ident".to_string(), format!("response_{}", question.id));

        // Set cardinality based on question type
        let rcardinality = match &question.question_type {
            QuestionType::MultipleChoice { .. } | QuestionType::TrueFalse { .. } => "Single",
            QuestionType::MultipleAnswer { .. } => "Multiple",
            _ => "Single",
        };
        response
            .attributes
            .insert("rcardinality".to_string(), rcardinality.to_string());

        // Add render_choice
        let mut render = Element::new("render_choice");
        if shuffle {
            render
                .attributes
                .insert("shuffle".to_string(), "Yes".to_string());
        }

        // Add response_label for each choice
        for choice in choices {
            let mut label = Element::new("response_label");
            label
                .attributes
                .insert("ident".to_string(), choice.id.clone());

            let mut material = Element::new("material");
            let mut mattext = Element::new("mattext");
            mattext
                .attributes
                .insert("texttype".to_string(), "text/html".to_string());
            mattext.children.push(XMLNode::Text(choice.text.clone()));
            material.children.push(XMLNode::Element(mattext));
            label.children.push(XMLNode::Element(material));

            render.children.push(XMLNode::Element(label));
        }

        response.children.push(XMLNode::Element(render));
        Ok(response)
    }

    fn build_response_str(&self, question: &Question) -> Result<Element> {
        let mut response = Element::new("response_str");
        response
            .attributes
            .insert("ident".to_string(), format!("response_{}", question.id));
        response
            .attributes
            .insert("rcardinality".to_string(), "Single".to_string());

        let mut render = Element::new("render_fib");

        // Set size based on question type
        match &question.question_type {
            QuestionType::ShortAnswer { .. } => {
                render
                    .attributes
                    .insert("columns".to_string(), "40".to_string());
            }
            QuestionType::Essay { .. } => {
                render
                    .attributes
                    .insert("rows".to_string(), "10".to_string());
                render
                    .attributes
                    .insert("columns".to_string(), "80".to_string());
            }
            _ => {}
        }

        response.children.push(XMLNode::Element(render));
        Ok(response)
    }

    fn build_response_num(&self, question: &Question) -> Result<Element> {
        let mut response = Element::new("response_num");
        response
            .attributes
            .insert("ident".to_string(), format!("response_{}", question.id));
        response
            .attributes
            .insert("rcardinality".to_string(), "Single".to_string());
        response
            .attributes
            .insert("numtype".to_string(), "Decimal".to_string());

        let render = Element::new("render_fib");
        response.children.push(XMLNode::Element(render));

        Ok(response)
    }

    fn build_resprocessing(&self, question: &Question) -> Result<Element> {
        let mut resprocessing = Element::new("resprocessing");

        let outcomes = self.build_outcomes(question)?;
        resprocessing.children.push(XMLNode::Element(outcomes));
        match &question.question_type {
            QuestionType::MultipleChoice { choices, .. } => {
                for choice in choices {
                    let condition = self.build_respcondition_mc(question, choice)?;
                    resprocessing.children.push(XMLNode::Element(condition));
                }
            }
            QuestionType::MultipleAnswer {
                choices,
                partial_credit,
            } => {
                for choice in choices {
                    let condition =
                        self.build_respcondition_ma(question, choice, *partial_credit)?;
                    resprocessing.children.push(XMLNode::Element(condition));
                }
            }
            QuestionType::ShortAnswer { answers, .. } => {
                for answer in answers {
                    let condition = self.build_respcondition_sa(question, answer)?;
                    resprocessing.children.push(XMLNode::Element(condition));
                }
            }
            _ => {
                let condition = self.build_respcondition_default(question)?;
                resprocessing.children.push(XMLNode::Element(condition));
            }
        }

        Ok(resprocessing)
    }

    fn build_outcomes(&self, question: &Question) -> Result<Element> {
        let mut outcomes = Element::new("outcomes");

        let mut decvar = Element::new("decvar");
        decvar
            .attributes
            .insert("maxvalue".to_string(), question.points.to_string());
        decvar
            .attributes
            .insert("minvalue".to_string(), "0".to_string());
        decvar
            .attributes
            .insert("varname".to_string(), "SCORE".to_string());
        decvar
            .attributes
            .insert("vartype".to_string(), "Decimal".to_string());

        outcomes.children.push(XMLNode::Element(decvar));
        Ok(outcomes)
    }

    fn build_respcondition_mc(&self, question: &Question, choice: &Choice) -> Result<Element> {
        let mut condition = Element::new("respcondition");
        condition
            .attributes
            .insert("continue".to_string(), "No".to_string());

        // Condition variable
        let mut condvar = Element::new("conditionvar");
        let mut varequal = Element::new("varequal");
        varequal
            .attributes
            .insert("respident".to_string(), format!("response_{}", question.id));
        varequal.children.push(XMLNode::Text(choice.id.clone()));
        condvar.children.push(XMLNode::Element(varequal));
        condition.children.push(XMLNode::Element(condvar));

        // Set score
        let mut setvar = Element::new("setvar");
        setvar
            .attributes
            .insert("action".to_string(), "Set".to_string());
        setvar
            .attributes
            .insert("varname".to_string(), "SCORE".to_string());
        let score = if choice.correct { question.points } else { 0.0 };
        setvar.children.push(XMLNode::Text(score.to_string()));
        condition.children.push(XMLNode::Element(setvar));

        if choice.correct && question.feedback.is_some() {
            let mut display = Element::new("displayfeedback");
            display
                .attributes
                .insert("linkrefid".to_string(), "correct".to_string());
            condition.children.push(XMLNode::Element(display));
        }

        Ok(condition)
    }

    fn build_respcondition_ma(
        &self,
        question: &Question,
        choice: &Choice,
        partial_credit: bool,
    ) -> Result<Element> {
        let mut condition = Element::new("respcondition");
        condition
            .attributes
            .insert("continue".to_string(), "Yes".to_string());

        // Condition variable
        let mut condvar = Element::new("conditionvar");
        let mut varequal = Element::new("varequal");
        varequal
            .attributes
            .insert("respident".to_string(), format!("response_{}", question.id));
        varequal.children.push(XMLNode::Text(choice.id.clone()));
        condvar.children.push(XMLNode::Element(varequal));
        condition.children.push(XMLNode::Element(condvar));

        // Set score (partial credit if enabled)
        let mut setvar = Element::new("setvar");
        setvar
            .attributes
            .insert("action".to_string(), "Add".to_string());
        setvar
            .attributes
            .insert("varname".to_string(), "SCORE".to_string());

        let score = if partial_credit && choice.correct {
            // TODO: should count actual number of correct answers for accurate partial credit
            question.points / 2.0
        } else if choice.correct {
            question.points
        } else {
            0.0
        };
        setvar.children.push(XMLNode::Text(score.to_string()));
        condition.children.push(XMLNode::Element(setvar));

        Ok(condition)
    }

    fn build_respcondition_sa(
        &self,
        question: &Question,
        answer: &AcceptableAnswer,
    ) -> Result<Element> {
        let mut condition = Element::new("respcondition");

        let mut condvar = Element::new("conditionvar");
        let mut varequal = Element::new("varequal");
        varequal
            .attributes
            .insert("respident".to_string(), format!("response_{}", question.id));
        varequal
            .attributes
            .insert("case".to_string(), "No".to_string());
        varequal.children.push(XMLNode::Text(answer.text.clone()));
        condvar.children.push(XMLNode::Element(varequal));
        condition.children.push(XMLNode::Element(condvar));

        let mut setvar = Element::new("setvar");
        setvar
            .attributes
            .insert("action".to_string(), "Set".to_string());
        setvar
            .attributes
            .insert("varname".to_string(), "SCORE".to_string());
        let score = question.points * answer.weight;
        setvar.children.push(XMLNode::Text(score.to_string()));
        condition.children.push(XMLNode::Element(setvar));

        Ok(condition)
    }

    fn build_respcondition_default(&self, _question: &Question) -> Result<Element> {
        let mut condition = Element::new("respcondition");

        let condvar = Element::new("conditionvar");
        condition.children.push(XMLNode::Element(condvar));

        let mut setvar = Element::new("setvar");
        setvar
            .attributes
            .insert("action".to_string(), "Set".to_string());
        setvar
            .attributes
            .insert("varname".to_string(), "SCORE".to_string());
        setvar.children.push(XMLNode::Text("0".to_string()));
        condition.children.push(XMLNode::Element(setvar));

        Ok(condition)
    }

    fn build_itemfeedback(&self, ident: &str, text: &str) -> Element {
        let mut feedback = Element::new("itemfeedback");
        feedback
            .attributes
            .insert("ident".to_string(), ident.to_string());

        let mut material = Element::new("material");
        let mut mattext = Element::new("mattext");
        mattext
            .attributes
            .insert("texttype".to_string(), "text/html".to_string());
        mattext.children.push(XMLNode::Text(text.to_string()));
        material.children.push(XMLNode::Element(mattext));
        feedback.children.push(XMLNode::Element(material));

        feedback
    }
}

impl Default for QtiBuilder {
    fn default() -> Self {
        Self::new()
    }
}
