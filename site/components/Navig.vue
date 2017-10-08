<template>
<nav class="navbar is-light">
  <div class="navbar-brand">
      <a href="./" class="navbar-item">mta-status</a>
    <div class="navbar-burger burger" v-bind:class="{ 'is-active': isActive }" data-target="navMenuTransparentExample" v-on:click="toggleMenu"  >
      <span></span>
      <span></span>
      <span></span>
    </div>
  </div>

  <div id="navMenuTransparentExample" class="navbar-menu" v-bind:class="{ 'is-active': isActive }">

    <div class="navbar-start">
    </div>

    <div class="navbar-end">
      <a class="navbar-item" href="https://github.com/toidiu/mta-status" target="_blank">
        <span class="icon" style="color: #333;">
          <i class="fa fa-lg fa-github"></i>
        </span>
      </a>
      <a href="./about" class="navbar-item">About</a>
    </div>

  </div>
</nav>
</template>

<script>
export default {
  data () {
    return {
      isActive: false
    }
  },
  methods: {
    toggleMenu: function () {
      this.isActive = !this.isActive
    }
  }
}
</script>

<style>
</style>
