<template>
  <section class="section">
    <div>

      <h1 class="title">
        mta-status
      </h1>

      <a v-on:click="refresh"  class="button is-primary">Refresh</a>
      </br>
      </br>
      <div>Last updated: {{ $store.state.mta.timestamp }}</div>

      </br>
      <div class="columns">
        <div class="column">
          <ul v-if="$store.state.mta.lines && $store.state.mta.lines.length">
            <li v-for="(line, idx) of $store.state.mta.lines" v-if="idx%2===0">
              <div><strong>{{ line.name }}</strong></div>
              <p class="status">{{ line.status }}</p>
            </li>
          </ul>
        </div>

        <div class="column">
          <ul v-if="$store.state.mta.lines && $store.state.mta.lines.length">
            <li v-for="(line, idx) of $store.state.mta.lines" v-if="idx%2===1">
              <div><strong>{{ line.name }}</strong></div>
              <p class="status">{{ line.status }}</p>
            </li>
          </ul>
        </div>
      </div>

    </div>
  </section>
</template>

<script>
import axios from 'axios'

export default {
  data () {
    return {
      timestamp: '',
      lines: [],
      errors: []
    }
  },
  methods: {
    refresh: async function (e) {
      try {
        let { data } = await axios.get('http://localhost:4000')
        this.$store.commit('setMta', data)
      } catch (e) {
        this.errors.push(e)
      }
    }
  },
  created () {
    axios.get('http://localhost:4000')
      .then(response => {
        // JSON responses are automatically parsed.
        this.lines = response.data.lines
        this.timestamp = response.data.timestamp
      })
      .catch(e => {
        this.errors.push(e)
      })
  },
  async fetch ({ store }) {
    try {
      let { data } = await axios.get('http://localhost:4000')
      store.commit('setMta', data)
    } catch (e) {
      this.errors.push(e)
    }
  }
}
</script>

<style>
.status {
  font-size: 14px
}


</style>
