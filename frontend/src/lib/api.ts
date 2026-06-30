import { useAppStore } from "@/store/useAppStore";

// const API_BASE = "http://localhost:8080/api"; // 로컬 테스트용
const API_BASE = "https://api.bangerdirect.com/api"; // 로컬 테스트용
// const API_BASE = "http://192.168.1.xxx:8080/api"; // 폰으로 테스트할 땐 본인 PC의 IP 입력 필수!

export const apiFetch = async (endpoint: string, options: RequestInit = {}) => {
  const token = useAppStore.getState().token;

  const headers = {
    "Content-Type": "application/json",
    ...(token && { Authorization: `Bearer ${token}` }),
    ...options.headers,
  };

  const response = await fetch(`${API_BASE}${endpoint}`, {
    ...options,
    headers,
  });

  if (!response.ok) {
    const errorData = await response.json().catch(() => ({}));
    throw new Error(errorData.error || "API 요청 실패");
  }

  return response.json();
};
