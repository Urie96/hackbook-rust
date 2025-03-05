// @generated automatically by Diesel CLI.

diesel::table! {
    article (id) {
        id -> Text,
        done -> Bool,
        publishDate -> Text,
        sectionId -> Text,
        title -> Text,
    }
}

diesel::table! {
    article_comment (id) {
        id -> Text,
        content -> Text,
        likeCount -> Integer,
        nickName -> Text,
        articleId -> Nullable<Text>,
        parentCommentId -> Nullable<Text>,
    }
}

diesel::table! {
    course (id) {
        id -> Text,
        brief -> Text,
        teacherName -> Text,
        teacherTitle -> Text,
        image -> Text,
        articleCount -> Integer,
        purchasedCount -> Text,
        done -> Bool,
        price -> Integer,
        title -> Text,
    }
}

diesel::table! {
    course_tend (courseId, userId) {
        courseId -> Text,
        userId -> Text,
        #[sql_name = "type"]
        type_ -> Text,
    }
}

diesel::table! {
    section (id) {
        id -> Text,
        courseId -> Text,
        title -> Text,
    }
}

diesel::table! {
    user (id) {
        id -> Text,
        role -> Text,
    }
}

diesel::table! {
    user_role (id) {
        user_id -> Text,
        role -> Integer,
        created_at -> BigInt,
        valid_before -> BigInt,
        id -> Integer,
    }
}

diesel::table! {
    user_study_info (id) {
        id -> Integer,
        course_id -> Text,
        article_id -> Text,
        last_study_at -> BigInt,
        study_percent -> Float,
        user_id -> Text,
    }
}

diesel::table! {
    ws_connect_info (id) {
        id -> Integer,
        user_id -> Text,
        start_at -> BigInt,
        end_at -> BigInt,
    }
}

diesel::joinable!(article -> section (sectionId));
diesel::joinable!(article_comment -> article (articleId));
diesel::joinable!(user_study_info -> article (article_id));
diesel::joinable!(user_study_info -> course (course_id));

diesel::allow_tables_to_appear_in_same_query!(
    article,
    article_comment,
    course,
    course_tend,
    section,
    user,
    user_role,
    user_study_info,
    ws_connect_info,
);
