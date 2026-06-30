-- 1. 유저 및 권한 (SUDO, ADMIN, SUPERVISOR, WORKER)
CREATE TABLE IF NOT EXISTS users (
    empno VARCHAR(20) PRIMARY KEY,
    password_hash VARCHAR(255) NOT NULL,
    role VARCHAR(20) NOT NULL, -- SUDO, ADMIN, SUPERVISOR, WORKER
    name VARCHAR(100) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    deleted_at TIMESTAMPTZ DEFAULT NULL -- Soft-delete용
);

-- 2. 인사(HR) 정보
CREATE TABLE IF NOT EXISTS employee_profiles (
    empno VARCHAR(20) PRIMARY KEY REFERENCES users(empno) ON DELETE CASCADE,
    phone VARCHAR(20),
    process_name_ko VARCHAR(100),
    process_name_es VARCHAR(100),
    hire_date DATE NOT NULL,
    is_resigned BOOLEAN DEFAULT FALSE,
    resign_date DATE
);

-- 3. 출석 및 식사 (위 스키마와 동일)
CREATE TABLE IF NOT EXISTS attendance_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    empno VARCHAR(20) NOT NULL REFERENCES users(empno),
    scanned_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    qr_jti VARCHAR(50) UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS meal_requests (
    id SERIAL PRIMARY KEY,
    empno VARCHAR(20) NOT NULL REFERENCES users(empno),
    target_date DATE NOT NULL,
    meal_type VARCHAR(20) NOT NULL,
    UNIQUE(empno, target_date, meal_type)
);