/* eslint-disable jsx-a11y/click-events-have-key-events */
/* eslint-disable @typescript-eslint/no-shadow */
import React, { useEffect, useRef, useState } from 'react'
import { Button } from 'react-bootstrap'
import styles from '../css/datePicker.module.css'
import { ICalender, getDate } from '../interfaces/types'
import TimePicker from './timePicker'

interface props {
  showDataAndTimePicker: boolean
  getDateAndTime: getDate
}

const DatePicker: React.FC<props> = ({
  showDataAndTimePicker,
  getDateAndTime,
}) => {
  const [date] = useState<Date>(new Date())
  const [selectedDate, setSelectedDate] = useState<Date>(new Date())
  const [day] = useState<number>(date.getDate())
  const [month, setMonth] = useState<number>(date.getMonth())
  const [year, setYear] = useState<number>(date.getFullYear())
  const months = useRef<string[]>([
    'January',
    'February',
    'March',
    'April',
    'May',
    'June',
    'July',
    'August',
    'September',
    'October',
    'November',
    'December',
  ])
  const [calTitle, setCalTitle] = useState<string>(
    `${months.current[month]} ${year}`
  )
  const [days, setDays] = useState<{ classes: string }[]>([])
  const [selected, setSelected] = useState<ICalender>({
    day,
    month,
    year,
  })
  const [time, setTime] = useState<string>('00:00:00')

  const populateDates = () => {
    const days: { classes: string }[] = []
    const classes: string[] = []
    let amountDays = 31

    if (month === 1) amountDays = 28
    if (month === 3 || month === 5 || month === 8 || month === 10)
      amountDays = 30

    for (let i = 0; i < amountDays; i++) {
      classes.push(styles.day)
      if (
        selected.day === i + 1 &&
        selected.year === year &&
        selected.month === month
      ) {
        classes.push(styles.selected)
      }
      days.push({ classes: classes.join(' ') })
      classes.length = 0
    }
    setDays(days)
    return days
  }

  useEffect(() => {
    setCalTitle(`${months.current[month]} ${year}`)
    setDays(populateDates())
    populateDates()
    return () => {}
  }, [month, year, day, selectedDate])

  const goToNextMonth = () => {
    setMonth((prev) => prev + 1)
    if (month + 1 > 11) {
      setMonth(0)
      setYear((prev) => prev + 1)
    }
    populateDates()
  }

  const goToPrevMonth = () => {
    setMonth((prev) => prev - 1)
    if (month - 1 < 0) {
      setMonth(11)
      setYear((prev) => prev - 1)
    }
    populateDates()
  }

  const selectDate = (day: number) => {
    const newSelect = new Date(`${year}-${month + 1}-${day + 1}`)
    setSelectedDate(newSelect)
    setSelected({ day: day + 1, month, year })
    populateDates()
  }

  const showTime = (date: string) => setTime(date)

  const selectDateAndTime = () => {
    const newSelect = new Date(`${year}-${month + 1}-${day}:${time}`)
    getDateAndTime(newSelect.getTime())
  }

  return (
    <div className={styles.container}>
      <div
        className={`${styles.datePicker} ${
          showDataAndTimePicker && styles.active
        }`}
      >
        <div
          role="textbox"
          onClick={(event) => event.stopPropagation()}
          className={`${styles.dates} d-grid`}
        >
          <div className={styles.month}>
            <div
              role="textbox"
              className={`${styles.arrows} ${styles.prevMth}`}
              onClick={goToPrevMonth}
            >
              &lt;
            </div>
            <div className={styles.mth}>{calTitle}</div>
            <div
              role="textbox"
              className={`${styles.arrows} ${styles.nextMth}`}
              onClick={goToNextMonth}
            >
              &gt;
            </div>
          </div>

          <div className={styles.days}>
            {days.map((item, index) => (
              <div
                role="textbox"
                className={item.classes}
                key={Math.random()}
                onClick={() => selectDate(index)}
              >
                {index + 1}
              </div>
            ))}
          </div>
          <TimePicker getTime={showTime} />
          <Button onClick={selectDateAndTime} variant="outline-light mb-1">
            Select
          </Button>
        </div>
      </div>
    </div>
  )
}

export default DatePicker
