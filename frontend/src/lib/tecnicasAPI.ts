import { apiFetch } from "./api";

import {
  Tool,
  CreateRentalReq,
  CreateToolReq,
  InventoryLogReq,
  Rental,
} from "@/types/tescnicas_herr";

export const tecnicas_herrAPI = {
  getTools: async (): Promise<Tool[]> => {
    // 변경지점: apiFetch 내부에서 Base URL 합성과 Token 주입이 자동으로 이루어짐
    return await apiFetch("/tools"); // 백엔드 라우터 경로에 맞게 엔드포인트 수정 필요
  },

  createTool: async (data: CreateToolReq): Promise<Tool> => {
    return await apiFetch("/tools", {
      method: "POST",
      body: JSON.stringify(data),
    });
  },
};
