<template>
  <section class="section">
    <div>
      <logo/>
      <h1 class="title">
        mta-status
      </h1>

      <div>
        <strong>mta-status</strong> is a project to parse the xml api that is provided by the mta and transform it into a nice json api.
      </div>

      </br>
      <div>
        The backend is written in <span class="rust-color">rust</span>, the frontend uses <span class="vue-color">vuejs</span> and its all hosted on <span class="github-color">github pages</span>
      </div>

      </br>
      <div>
        More information can be found at the <a href="http://github.com/toidiu/mta-status">
        <i class="fa fa-lg fa-github"></i>
        </a> page.
      </div>
    </div>
  </section>
</template>

<script>

export default {
  components: {
  }
}
</script>

<style>
.rust-color{
  color: #DEA584;
}
.vue-color{
  color: #41B883;
}
.github-color{
  color: #387FB5;
}


</style>
