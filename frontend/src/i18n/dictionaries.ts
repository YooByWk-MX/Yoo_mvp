import { Dictionary, Lang } from "@/types/jig";

const dictionaries: Record<Lang, Dictionary> = {
  ko: {
    main: {
      title: "YURA 통합 마스터 포탈",
      portalBtn: "지그(Jig) 관리 시스템으로 이동 ➔",
    },
    jig: {
      title: "지그 마스터 관리 시스템",
      formTitle: "신규 지그 등록",
      provLabel: "공급처 (Prov - 여러 줄 입력 가능)",
      qtyLabel: "수량 (CANTIDAD)",
      tableroLabel: "작업대 (Tablero)",
      typesLabel: "유형 (Types)",
      typeEnsamblaje: "조립 (Ensamblaje)", // 추가됨
      typePrueba: "검증 (Prueba)", // 추가됨
      detailsLabel: "기타 상세 (Pin / Color / MF / Note)",
      submitBtn: "시스템 등록 (PostgreSQL)",
      tableNo: "번호",
      tableProv: "공급처 (Prov)",
      tableQty: "수량",
      tableTablero: "작업대",
      tableTypes: "유형",
      loading: "PostgreSQL 데이터 동기화 중...",
    },
  },
  es: {
    main: {
      title: "Portal Maestro Integrado YURA",
      portalBtn: "Ir al Sistema de Gestión de Jig ➔",
    },
    jig: {
      title: "Sistema de Gestión de Jig Maestro",
      formTitle: "Registrar Nuevo Jig",
      provLabel: "Proveedor (Prov - Multilínea)",
      qtyLabel: "Cantidad (CANTIDAD)",
      tableroLabel: "Tablero",
      typesLabel: "Tipos (Types)",
      typeEnsamblaje: "Ensamblaje", // 추가됨
      typePrueba: "Prueba / Inspección", // 추가됨
      detailsLabel: "Detalles Adicionales (Pin / Color / MF / Note)",
      submitBtn: "Registrar en el Sistema (PostgreSQL)",
      tableNo: "Nº",
      tableProv: "Proveedor (Prov)",
      tableQty: "CANTIDAD",
      tableTablero: "Tablero",
      tableTypes: "Tipos",
      loading: "Sincronizando datos de PostgreSQL...",
    },
  },
};

export const getDictionary = (lang: Lang): Dictionary =>
  dictionaries[lang] || dictionaries.ko;
