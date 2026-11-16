import { useEffect, useState } from 'react'

const batchSize = 3

export function useProgressiveRows<T extends { id: string }>(rows: T[]) {
  const key = JSON.stringify(rows.map((row) => row.id))
  const total = rows.length
  const [progress, setProgress] = useState(() => ({ key, count: Math.min(batchSize, total) }))
  let count = progress.count
  if (progress.key !== key) {
    const preservesPrefix = (JSON.parse(progress.key) as string[]).every((id, index) => id === rows[index]?.id)
    count = Math.min(rows.length, preservesPrefix ? Math.max(batchSize, count) : batchSize)
    setProgress({ key, count })
  }
  useEffect(() => {
    if (count >= total) return
    let cancelled = false
    const scheduler = (globalThis as typeof globalThis & { scheduler?: { yield(): Promise<void> } }).scheduler
    let timer: ReturnType<typeof setTimeout> | undefined
    const pause = scheduler?.yield
      ? scheduler.yield()
      : new Promise<void>((resolve) => {
          timer = setTimeout(resolve, 0)
        })
    void pause.then(() => {
      if (!cancelled) setProgress({ key, count: Math.min(count + batchSize, total) })
    })
    return () => {
      cancelled = true
      clearTimeout(timer)
    }
  }, [key, count, total])
  return { rows: rows.slice(0, count), revealing: count < rows.length }
}
