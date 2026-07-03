-- 1. 공구 마스터 테이블 (추후 QR 그룹핑 대비)
CREATE TABLE IF NOT EXISTS tools (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    current_qty INT DEFAULT 0, -- 현재고 (입출고 시 자동 갱신)
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- 2. 입출고 대장 (Control de Entradas y Salidas)
CREATE TABLE IF NOT EXISTS tool_inventory_logs (
    id SERIAL PRIMARY KEY,
    tool_id INT NOT NULL REFERENCES tools(id),
    log_type VARCHAR(10) NOT NULL, -- 'IN'(입고) / 'OUT'(출고/소모)
    qty INT NOT NULL,
    balance INT NOT NULL, -- 처리 후 잔량 (추적용)
    supplier VARCHAR(100), -- 구매처
    unit_price DECIMAL(10, 2), -- 개당 가격
    purchase_date DATE, -- 구매/품의 일자
    log_date DATE NOT NULL, -- 실제 입/출고 일자
    manager_empno VARCHAR(20) NOT NULL REFERENCES users(empno), -- 서명(담당 관리자)
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- 3. 공구 대여 이력 (Préstamos)
CREATE TABLE IF NOT EXISTS tool_rentals (
    id SERIAL PRIMARY KEY,
    tool_id INT NOT NULL REFERENCES tools(id),
    borrower_empno VARCHAR(20), -- 사번 (사내 직원일 경우)
    borrower_name VARCHAR(100) NOT NULL, -- 대여받는 사람 이름
    borrower_team VARCHAR(50), -- 빌려가는 소속 팀 (타 팀 지원용)
    rent_start TIMESTAMPTZ NOT NULL DEFAULT NOW(), -- 대여 시작
    rent_end TIMESTAMPTZ, -- 반납 완료 시각
    status VARCHAR(20) NOT NULL DEFAULT 'RENTED', -- RENTED(대여중), RETURNED(반납완료)
    manager_empno VARCHAR(20) NOT NULL REFERENCES users(empno) -- 불출 담당자
);