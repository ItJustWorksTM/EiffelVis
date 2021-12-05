/* eslint-disable jsx-a11y/control-has-associated-label */
/* eslint-disable jsx-a11y/click-events-have-key-events */
import React, { useEffect, useRef, useState } from 'react'
import styles from '../css/timePicker.module.css'
import { formatTime } from '../helpers/dateFromatter'
import { _getTime } from '../interfaces/types'

interface props {
  getTime: _getTime
}

const TimePicker: React.FC<props> = ({ getTime }) => {
  const { current: currentTime } = useRef(new Date())
  const [hour, setHour] = useState(currentTime.getHours())
  const [minute, setMinute] = useState(currentTime.getMinutes())
  const [second, setSecond] = useState(currentTime.getSeconds())
  const [time, setTime] = useState('00:00:00')

  const selectedTime = (t: string) => getTime(t)

  useEffect(() => {
    setTime(`${formatTime(hour)}:${formatTime(minute)}:${formatTime(second)}`)
    selectedTime(time)
  }, [hour, minute, second, time])

  const hourChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (+e.target.value > 23) {
      setHour(() => 23)
    } else if (+e.target.value < 0) {
      setHour(0)
    } else if (e.target.value === '') {
      setHour(0)
    } else {
      setHour(+e.target.value)
    }
  }

  const minuteChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (+e.target.value > 59) {
      setMinute(59)
    } else if (+e.target.value < 0) {
      setMinute(0)
    } else if (e.target.value === '') {
      setMinute(0)
    } else {
      setMinute(+e.target.value)
    }
  }

  const secondChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (+e.target.value > 59) {
      setSecond(59)
    } else if (+e.target.value < 0) {
      setSecond(0)
    } else if (e.target.value === '') {
      setSecond(0)
    } else {
      setSecond(+e.target.value)
    }
  }

  const hourUp = () => {
    setHour((prev) => prev + 1)
    if (hour + 1 > 23) {
      setHour(0)
    }
  }
  const hourDown = () => {
    setHour((prev) => prev - 1)
    if (hour - 1 < 0) {
      setHour(23)
    }
  }

  const minuteUp = () => {
    setMinute((prev) => prev + 1)
    if (minute + 1 > 59) {
      setMinute(0)
      hourUp()
    }
  }
  const minuteDown = () => {
    setMinute((prev) => prev - 1)
    if (minute - 1 < 0) {
      setMinute(59)
      hourDown()
    }
  }

  const secondUp = () => {
    setSecond((prev) => prev + 1)
    if (second + 1 > 59) {
      setSecond(0)
      minuteUp()
    }
  }
  const secondDown = () => {
    setSecond((prev) => prev - 1)
    if (second - 1 < 0) {
      setSecond(59)
      minuteDown()
    }
  }

  return (
    <div className={styles.timePicker} data-time={time}>
      <div className={styles.hour}>
        <div role="textbox" onClick={hourUp} className={styles.hrUp} />
        <input
          placeholder="00"
          type="number"
          className={styles.hr}
          value={hour}
          onChange={hourChange}
        />
        <div role="textbox" onClick={hourDown} className={styles.hrDown} />
      </div>

      <div className={styles.separator}>:</div>

      <div className={styles.minute}>
        <div role="textbox" onClick={minuteUp} className={styles.minUp} />
        <input
          placeholder="00"
          type="number"
          className={styles.min}
          value={minute}
          onChange={minuteChange}
        />
        <div role="textbox" onClick={minuteDown} className={styles.minDown} />
      </div>

      <div className={styles.separator}>:</div>

      <div className={styles.second}>
        <div role="textbox" onClick={secondUp} className={styles.secUp} />
        <input
          placeholder="00"
          type="number"
          className={styles.sec}
          value={second}
          onChange={secondChange}
        />
        <div role="textbox" onClick={secondDown} className={styles.secDown} />
      </div>
    </div>
  )
}

export default TimePicker
