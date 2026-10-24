document.addEventListener("nav", () => {
  const filter = document.querySelector<HTMLSelectElement>(".feature-filter")
  if (!filter) return
  const change = () => window.location.assign(filter.value)
  filter.addEventListener("change", change)
  window.addCleanup(() => filter.removeEventListener("change", change))
})
