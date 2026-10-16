let retryCount = 0;

async function start(retry = false) {
  try {
    const modulePath = retry
      ? `./pkg/recompiled.js?retry=${retryCount++}`
      : './pkg/recompiled.js';
    const { default: init } = await import(modulePath);
    await init();
  } catch (error) {
    console.error('The Rust archive failed to start:', error);
    showBootstrapError();
  }
}

function showBootstrapError() {
  const app = document.getElementById('app');
  if (!app) return;

  const messages = {
    en: ['Archive unavailable', 'The archive could not start', 'The site could not finish loading. Check your connection and try again.', 'Try again', 'Reload page'],
    es: ['Archivo no disponible', 'No se pudo iniciar el archivo', 'No se pudo completar la carga del sitio. Comprueba tu conexión e inténtalo de nuevo.', 'Intentar de nuevo', 'Recargar la página'],
    fr: ['Archive indisponible', 'L’archive n’a pas pu démarrer', 'Le site n’a pas pu terminer son chargement. Vérifiez votre connexion puis réessayez.', 'Réessayer', 'Recharger la page'],
    pt: ['Arquivo indisponível', 'Não foi possível iniciar o arquivo', 'Não foi possível concluir o carregamento do site. Verifique sua conexão e tente novamente.', 'Tentar novamente', 'Recarregar a página'],
    ja: ['アーカイブを利用できません', 'アーカイブを起動できませんでした', 'サイトを読み込めませんでした。接続を確認して、もう一度お試しください。', '再試行', 'ページを再読み込み'],
    zh: ['档案不可用', '无法启动档案', '网站未能完成加载。请检查网络连接后重试。', '重试', '刷新页面'],
  };
  const locale = document.documentElement.lang;
  const [label, title, description, retry, reload] = messages[locale] || messages.en;
  app.innerHTML = `<main class="page-loader" aria-label="${label}"><div class="page-loader-card" role="alert" aria-live="assertive" aria-atomic="true"><span class="page-loader-brand">recompiled<span>.</span></span><p class="page-loader-copy"><strong>${title}</strong>${description}</p><div style="display:flex;flex-wrap:wrap;justify-content:center;gap:8px;margin-top:8px"><button class="theme-toggle" type="button" data-retry>${retry}</button><button class="theme-toggle" type="button" data-reload>${reload}</button></div></div></main>`;
  app.querySelector('[data-retry]').addEventListener('click', () => start(true));
  app.querySelector('[data-reload]').addEventListener('click', () => window.location.reload());
}

start();
