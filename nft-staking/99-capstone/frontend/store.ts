import { create } from "zustand";

interface State {
  isShowSpinner: boolean;
  setIsShowSpinner: (newValue: boolean) => void;
}

export const useStore = create<State>()((set) => ({
  isShowSpinner: true,
  setIsShowSpinner: (newValue) => set({ isShowSpinner: newValue }),
}));
