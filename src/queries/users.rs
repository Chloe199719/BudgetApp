pub const USER_AND_USER_PROFILE_QUERY: &str =
    "SELECT 
 u.id as u_id,
 u.email as u_email,
    u.password as u_password,
    u.display_name as u_display_name,
    u.unique_name as u_unique_name,
    u.is_active as u_is_active,
    u.is_staff as u_is_staff,
    u.is_superuser as u_is_superuser,
    u.thumbnail as u_thumbnail,
    u.data_joined as u_data_joined,
    p.id as p_id,
    p.phone_number as p_phone_number,
    p.birth_date as p_birth_date,
    p.github_link as p_github_link,
    p.about_me as p_about_me,
    p.pronouns as p_pronouns,
    p.avatar_link as p_avatar_link
FROM
    users u
        LEFT JOIN
    user_profile p ON u.id = p.user_id = u.id
WHERE
u.is_active = true AND ";
