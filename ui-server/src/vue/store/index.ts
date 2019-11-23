import Vue from 'vue';
import Vuex from 'vuex';

const fetchItem = (name: string): Promise<string> => new Promise((resolve, reject): void => {

});

export const createStore = (): any => new Vuex.Store({
  state: () => ({
    items: {}
  }),

  actions: {
    fetchItem ({ commit }: any, id: any) {
      // return the Promise via `store.dispatch()` so that we know
      // when the data has been fetched
      return fetchItem(id).then((item: any): any => {
        commit('setItem', { id, item })
      })
    }
  },

  mutations: {
    setItem (state: any, { id, item }: any) {
      Vue.set(state.items, id, item)
    }
  }
});