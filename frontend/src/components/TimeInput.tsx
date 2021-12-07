import React, { useState } from 'react'
import { Form, FormControl, InputGroup } from 'react-bootstrap'
import { ClockHistory } from 'react-bootstrap-icons'
import styles from '../css/input.module.css'
import DatePicker from './DatePicker'

interface IProps {
  title: string
  placeHolder: string
  handleDateChange: Function
}
const TimeInput: React.FC<IProps> = ({
  title,
  placeHolder,
  handleDateChange,
}) => {
  const [showDatePicker, setShowDatePicker] = useState(false)
  const [unixTime, setUnixTime] = useState('')

  const showTime = (date: number) => {
    setUnixTime(date.toString())
    setShowDatePicker(false)
    handleDateChange(date.toString())
  }
  const showDate = () => {
    handleDateChange(unixTime)
    setShowDatePicker((prev) => !prev)
  }
  return (
    <Form.Group className="mb-2">
      <Form.Label htmlFor={title.trim()}>{title}</Form.Label>
      <InputGroup size="sm">
        <FormControl
          autoComplete="off"
          value={unixTime}
          size="sm"
          id={title.trim()}
          placeholder={placeHolder}
          onClick={showDate}
          onChange={(e: any) => {
            setUnixTime(e.target.value)
          }}
        />
        <InputGroup.Text className={styles.inputIcon} onClick={showDate}>
          <ClockHistory />
          <DatePicker
            getDateAndTime={showTime}
            showDataAndTimePicker={showDatePicker}
          />
        </InputGroup.Text>
      </InputGroup>
    </Form.Group>
  )
}

export default TimeInput
