export interface Tool {
  id: number;
  name: string;
  current_qty: number;
}

export interface CreateToolReq {
  name: string;
}

// 백엔드 InventoryLogReq 모델 매핑
export interface InventoryLogReq {
  tool_id: number;
  log_type: "IN" | "OUT";
  qty: number;
  supplier?: string;
  unit_price?: number;
  purchase_date?: string; // YYYY-MM-DD
  log_date: string; // YYYY-MM-DD
}

// 백엔드 Rental 모델 매핑
export interface Rental {
  id: number;
  tool_id: number;
  tool_name: string;
  borrower_empno?: string;
  borrower_name: string;
  borrower_team?: string;
  rent_start: string;
  rent_end?: string;
  status: string; // "RENTED" | "RETURNED"
}

export interface CreateRentalReq {
  tool_id: number;
  borrower_empno?: string;
  borrower_name: string;
  borrower_team?: string;
}
