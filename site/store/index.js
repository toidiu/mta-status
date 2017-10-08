import Vuex from 'vuex'

const createStore = () => {
  return new Vuex.Store({
    state: {
      counter: 0,
      mta: 'none'
    },
    mutations: {
      increment (state) {
        state.counter++
      },
      setMta (state, data) {
        state.mta = data
      }
    }
  })
}

export default createStore
