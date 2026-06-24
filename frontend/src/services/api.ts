import { Jig, CreateJigDto } from "@/types/jig";

// const API_BASE = "http://localhost:8080/api";
const API_BASE = "http://192.168.1.66:8080/api";
export const jigApi = {
  getAll: async (): Promise<Jig[]> => {
    const res = await fetch(`${API_BASE}/jigs`, { cache: "no-store" });
    if (!res.ok) throw new Error("Failed to fetch jigs");
    return res.json();
  },

  create: async (data: CreateJigDto): Promise<Jig> => {
    const res = await fetch(`${API_BASE}/jigs`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(data),
    });
    if (!res.ok) throw new Error("Failed to create jig");
    return res.json();
  },
};
