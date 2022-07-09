try {
  var preference_query = window.matchMedia('(prefers-color-scheme: dark)');
  function checkPreference(query) {
    if (query.matches) {
      // A dark color scheme preference is set so we add the class from our html element
      document.documentElement.classList.add('dark');
    } else {
      // No dark color scheme preference is set so we remove the class from our html element
      document.documentElement.classList.remove('dark');
    }
  }
  checkPreference(preference_query);
  preference_query.addEventListener("change", checkPreference);

  // if (localStorage.theme === 'dark' || (!('theme' in localStorage) && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
  //   document.documentElement.classList.add('dark')
  // } else {
  //   document.documentElement.classList.remove('dark')
  // }
} catch (_) { }
