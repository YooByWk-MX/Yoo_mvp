// src/store/useAppStore.ts
import { create } from "zustand";
import { persist } from "zustand/middleware";

type Lang = "ko" | "es";

interface AppState {
  lang: Lang;
  token: string | null;
  role: string | null;
  empno: string | null;
  setLang: (lang: Lang) => void;
  setAuth: (token: string, role: string, empno: string) => void;
  logout: () => void;
}

export const useAppStore = create<AppState>()(
  persist(
    (set) => ({
      lang: "ko",
      token: null,
      role: null,
      empno: null,
      setLang: (lang) => set({ lang }),
      setAuth: (token, role, empno) => set({ token, role, empno }),
      logout: () => set({ token: null, role: null, empno: null }),
    }),
    { name: "yura-app-storage" },
  ),
);

// 다국어 사전 (I18n)
export const dict = {
  ko: {
    login: "로그인",
    empno: "사번 (Emp No)",
    password: "비밀번호",
    admin_qr: "출석 QR (관리자용)",
    worker_scan: "QR 출석 스캔",
    meal_req: "식사 신청",
    lunch: "점심",
    dinner: "저녁",
    jig_manage: "지그 관리",
    add_jig: "지그 추가",
    loading: "로딩중...",
    success: "성공적으로 처리되었습니다.",
  },
  es: {
    login: "Iniciar sesión",
    empno: "No. Empleado",
    password: "Contraseña",
    admin_qr: "QR de Asistencia (Admin)",
    worker_scan: "Escanear QR",
    meal_req: "Solicitud de Comida",
    lunch: "Almuerzo",
    dinner: "Cena",
    jig_manage: "Gestión de Jig",
    add_jig: "Agregar Jig",
    loading: "Cargando...",
    success: "Procesado exitosamente.",
  },
};
