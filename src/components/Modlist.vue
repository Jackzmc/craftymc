<template>
  <table class="table is-fullwidth">
  <thead>
    <tr>
      <th>Name</th>
      <th>Author</th>
      <th>File</th>
      <th>Action</th>
      <th>Enabled</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Applied Energistics</td>
      <td>Tom Brady</td>
      <td>ae2.jar</td>
      <td>
        <div class="buttons">
        <a class="button is-small is-white"><Icon :icon="['fas', 'download']" /></a>
        <a class="button is-small is-white"><Icon :icon="['fas', 'trash']" /></a>
        </div>
      </td>
      <td>
        <input id="mod1-active" type="checkbox" name="switchRoundedDefault" class="switch is-rounded is-info" checked="checked">
        <label for="mod1-active"></label>
      </td>
    </tr>
    <!--<tr v-for="mod in modpack.mods" :key="mod.id">

    </tr>-->
  </tbody>
</table>
</template>
