-- 1. 유저 및 권한
CREATE TABLE IF NOT EXISTS users (
    empno VARCHAR(20) PRIMARY KEY,
    password_hash VARCHAR(255) NOT NULL,
    role VARCHAR(20) NOT NULL DEFAULT 'WORKER',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 2. 인사(HR) 정보
CREATE TABLE IF NOT EXISTS employee_profiles (
    empno VARCHAR(20) PRIMARY KEY REFERENCES users(empno) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    phone VARCHAR(20),
    gender VARCHAR(10),
    hire_date DATE NOT NULL,
    job_grade VARCHAR(50),
    process_name_ko VARCHAR(100),
    process_name_es VARCHAR(100),
    municipio VARCHAR(100),
    transport VARCHAR(50),
    nationality VARCHAR(50) DEFAULT '멕시코',
    is_resigned BOOLEAN DEFAULT FALSE,
    resign_date DATE
);

-- 3. 근무 조 이력
CREATE TABLE IF NOT EXISTS shift_history (
    id SERIAL PRIMARY KEY,
    empno VARCHAR(20) NOT NULL REFERENCES users(empno) ON DELETE CASCADE,
    shift_type VARCHAR(20) NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE
);

-- 4. 출석(QR) 기록
CREATE TABLE IF NOT EXISTS attendance_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    empno VARCHAR(20) NOT NULL REFERENCES users(empno),
    scanned_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    qr_jti VARCHAR(50) UNIQUE NOT NULL
);

-- 5. 식사 신청
CREATE TABLE IF NOT EXISTS meal_requests (
    id SERIAL PRIMARY KEY,
    empno VARCHAR(20) NOT NULL REFERENCES users(empno),
    target_date DATE NOT NULL,
    meal_type VARCHAR(20) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(empno, target_date, meal_type)
);