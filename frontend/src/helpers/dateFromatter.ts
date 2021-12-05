export const formatDate = (d: Date) => {
  const year = d.getFullYear()
  let day: number | string = d.getDate()
  let month: number | string = d.getMonth() + 1

  if (day < 10) {
    day = `0${day}`
  }
  if (month < 10) {
    month = `0${month}`
  }

  return `${year} - ${month} - ${day}`
}

export const formatTime = (t: number) => {
  let formatted: number | string = t
  if (t < 10) {
    formatted = `0${t}`
  }
  return formatted
}
