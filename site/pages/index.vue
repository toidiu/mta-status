<template>
  <section class="section">
    <div>

      <h1 class="title">
        mta-status
      </h1>

      <a v-on:click="refresh"  class="button is-primary">
        Refresh
      </a>
      <span v-if="this.slowDown === true">
        what's the rush..
      </span>

      </br>
      </br>
      <div>Last called: {{ called.toLocaleDateString() }} {{ called.toLocaleTimeString() }}</div>
      <div>Last updated by MTA: {{ $store.state.mta.timestamp }}</div>

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
      called: null,
      errors: [],
      slowDown: false
    }
  },
  methods: {
    allowRefresh: function (e) {
      this.slowDown = false
    },
    refresh: async function (e) {
      var now = new Date()
      if (now - this.called > 1000) {
        this.slowDown = false
        this.called = new Date()
        try {
          let { data } = await axios.get('http://pi.toidiu.com:1000')
          this.$store.commit('setMta', data)
        } catch (e) {
          this.errors.push(e)
        }
      } else if (!this.slowDown) {
        this.slowDown = true
        setTimeout(this.allowRefresh, 1000)
      }
    }
  },
  async created () {
    this.refresh()
  }
}
</script>

<style>
.status {
  font-size: 14px
}

</style>
