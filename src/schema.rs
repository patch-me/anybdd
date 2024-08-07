// @generated automatically by Diesel CLI.

diesel::table! {
    project_tasks (project_id, task_id) {
        project_id -> Integer,
        task_id -> Integer,
    }
}

diesel::table! {
    projects (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        description -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    tags (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    task_tags (task_id, tag_id) {
        task_id -> Integer,
        tag_id -> Integer,
    }
}

diesel::table! {
    tasks (id) {
        id -> Integer,
        #[max_length = 255]
        title -> Varchar,
        description -> Text,
        due_date -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    user_projects (user_id, project_id) {
        user_id -> Integer,
        project_id -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        #[max_length = 255]
        username -> Varchar,
        password -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(project_tasks -> projects (project_id));
diesel::joinable!(project_tasks -> tasks (task_id));
diesel::joinable!(task_tags -> tags (tag_id));
diesel::joinable!(task_tags -> tasks (task_id));
diesel::joinable!(user_projects -> projects (project_id));
diesel::joinable!(user_projects -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    project_tasks,
    projects,
    tags,
    task_tags,
    tasks,
    user_projects,
    users,
);
