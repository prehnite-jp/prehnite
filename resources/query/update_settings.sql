INSERT INTO settings(setting_key, setting_value)
VALUES (?1, ?2)
ON CONFLICT DO UPDATE SET setting_value = ?2;