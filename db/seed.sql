-- USERS
INSERT INTO users (id, name, email, password_hash, role, created_at)
VALUES
  (gen_random_uuid(), 'Admin1', 'admin1@example.com', '123456', 'admin', NOW()),
  (gen_random_uuid(), 'Analyst1', 'analyst1@example.com', '123456', 'analyst', NOW()),
  (gen_random_uuid(), 'Viewer1', 'viewer1@example.com', '123456', 'viewer', NOW());

-- TRANSACTIONS (use subquery to link users)
-- =========================
-- ADMIN1 TRANSACTIONS
-- =========================

-- Income
INSERT INTO transactions (id, user_id, amount, type, category, description, created_at)
SELECT gen_random_uuid(), u.id, 5000, 'income', 'salary', 'monthly salary', NOW() - INTERVAL '30 days'
FROM users u WHERE u.email = 'admin1@example.com';

INSERT INTO transactions (id, user_id, amount, type, category, description, created_at)
SELECT gen_random_uuid(), u.id, 2000, 'income', 'freelance', 'project payment', NOW() - INTERVAL '15 days'
FROM users u WHERE u.email = 'admin1@example.com';

-- Expenses
INSERT INTO transactions (id, user_id, amount, type, category, description, created_at)
SELECT gen_random_uuid(), u.id, 1200, 'expense', 'food', 'groceries', NOW() - INTERVAL '10 days'
FROM users u WHERE u.email = 'admin1@example.com';

INSERT INTO transactions (id, user_id, amount, type, category, description, created_at)
SELECT gen_random_uuid(), u.id, 800, 'expense', 'transport', 'cab rides', NOW() - INTERVAL '8 days'
FROM users u WHERE u.email = 'admin1@example.com';

INSERT INTO transactions (id, user_id, amount, type, category, description, created_at)
SELECT gen_random_uuid(), u.id, 1500, 'expense', 'shopping', 'clothes', NOW() - INTERVAL '5 days'
FROM users u WHERE u.email = 'admin1@example.com';

INSERT INTO transactions (id, user_id, amount, type, category, description, created_at)
SELECT gen_random_uuid(), u.id, 300, 'expense', 'entertainment', 'movie night', NOW() - INTERVAL '3 days'
FROM users u WHERE u.email = 'admin1@example.com';

INSERT INTO transactions (id, user_id, amount, type, category, description, created_at)
SELECT gen_random_uuid(), u.id, 700, 'expense', 'utilities', 'electricity bill', NOW() - INTERVAL '2 days'
FROM users u WHERE u.email = 'admin1@example.com';

-- =========================
-- ANALYST1 TRANSACTIONS
-- =========================

-- Income
INSERT INTO transactions (id, user_id, amount, type, category, description, created_at)
SELECT gen_random_uuid(), u.id, 4000, 'income', 'salary', 'monthly salary', NOW() - INTERVAL '30 days'
FROM users u WHERE u.email = 'analyst1@example.com';

-- Expenses
INSERT INTO transactions (id, user_id, amount, type, category, description, created_at)
SELECT gen_random_uuid(), u.id, 900, 'expense', 'food', 'restaurant', NOW() - INTERVAL '12 days'
FROM users u WHERE u.email = 'analyst1@example.com';

INSERT INTO transactions (id, user_id, amount, type, category, description, created_at)
SELECT gen_random_uuid(), u.id, 600, 'expense', 'transport', 'fuel', NOW() - INTERVAL '7 days'
FROM users u WHERE u.email = 'analyst1@example.com';

INSERT INTO transactions (id, user_id, amount, type, category, description, created_at)
SELECT gen_random_uuid(), u.id, 1100, 'expense', 'rent', 'monthly rent', NOW() - INTERVAL '5 days'
FROM users u WHERE u.email = 'analyst1@example.com';

INSERT INTO transactions (id, user_id, amount, type, category, description, created_at)
SELECT gen_random_uuid(), u.id, 250, 'expense', 'entertainment', 'games', NOW() - INTERVAL '1 day'
FROM users u WHERE u.email = 'analyst1@example.com';