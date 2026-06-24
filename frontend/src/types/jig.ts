export interface Jig {
  no: number;
  prov: string;
  cantidad: number;
  tablero: string | null;
  color: string | null;
  mf: string | null;
  types: string | null;
  pin: string | null;
  note: string | null;
}

export type CreateJigDto = Omit<Jig, "no">;

export type Lang = "ko" | "es";

export interface Dictionary {
  main: {
    title: string;
    portalBtn: string;
  };
  jig: {
    title: string;
    formTitle: string;
    provLabel: string;
    qtyLabel: string;
    tableroLabel: string;
    typesLabel: string;
    // [추가됨] 드롭다운 옵션 다국어 타입
    typeEnsamblaje: string;
    typePrueba: string;
    detailsLabel: string;
    submitBtn: string;
    tableNo: string;
    tableProv: string;
    tableQty: string;
    tableTablero: string;
    tableTypes: string;
    loading: string;
  };
}
