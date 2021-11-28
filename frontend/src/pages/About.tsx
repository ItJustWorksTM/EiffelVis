import React, { useState } from 'react'
import { Form, FormControl, InputGroup } from 'react-bootstrap'
import DatePicker from '../components/DatePicker'

const About: React.FC = () => {
  const [showDatePicker, setShowDatePicker] = useState(false)
  const [unixTime, setUnixTime] = useState('')

  const showTime = (date: number) => {
    setUnixTime(date.toString())
    setShowDatePicker(false)
  }
  const showDate = () => setShowDatePicker(true)

  return (
    <div>
      <div className="w-50 mb-5 mx-auto">
        <Form.Label htmlFor="inlineFormInputGroupDate" visuallyHidden>
          date
        </Form.Label>
        <InputGroup>
          <InputGroup.Text onClick={showDate}>Pick Date</InputGroup.Text>
          <FormControl
            id="inlineFormInputGroupDate"
            placeholder="date"
            value={unixTime}
          />
        </InputGroup>
        <DatePicker
          getDateAndTime={showTime}
          showDataAndTimePicker={showDatePicker}
        />
      </div>
    </div>
  )
}

export default About
