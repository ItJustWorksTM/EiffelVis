<script lang="ts">
  import { onMount, createEventDispatcher } from 'svelte';
  import TimePicker from './TimePicker.svelte';

  const dispatch = createEventDispatcher();

  export let showDataAndTimePicker = false;

  const formatDate = (d: Date) => {
    const year = d.getFullYear();
    let day: number | string = d.getDate();
    let month: number | string = d.getMonth() + 1;

    if (day < 10) {
      day = `0${day}`;
    }
    if (month < 10) {
      month = `0${month}`;
    }

    return `${year} - ${month} - ${day}`;
  };

  const date = new Date();
  let selectedDate = new Date();
  let day = date.getDate();
  let month = date.getMonth();
  let year = date.getFullYear();
  const months = [
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
  ];

  $: calTitle = `${months[month]} ${year}`;
  let dateFormatted = formatDate(date);
  let days: { classes: string }[] = [];
  let selected = {
    day,
    month,
    year,
  };
  let time = '00:00:00';

  const populateDates = () => {
    const d: { classes: string }[] = [];
    const classes: string[] = [];
    let amountDays = 31;

    if (month === 1) amountDays = 28;
    if (month === 3 || month === 5 || month === 8 || month === 10)
      amountDays = 30;

    for (let i = 0; i < amountDays; i++) {
      classes.push('day');
      if (
        selected.day === i + 1 &&
        selected.year === year &&
        selected.month === month
      ) {
        classes.push('selected');
      }
      d.push({ classes: classes.join(' ') });
      classes.length = 0;
    }
    days = d;
    return d;
  };

  onMount(() => {
    calTitle = `${months[month]} ${year}`;
    days = populateDates();
    populateDates();
  });

  const goToNextMonth = () => {
    month++;
    if (month > 11) {
      month = 0;
      year++;
    }
    populateDates();
  };

  const goToPrevMonth = () => {
    month--;
    if (month < 0) {
      month = 11;
      year--;
    }
    populateDates();
  };

  const selectDate = (day: number) => {
    selectedDate = new Date(`${year}-${month + 1}-${day + 1}:${time}`);
    selected = { day: day + 1, month, year };
    dateFormatted = formatDate(selectedDate);
    populateDates();
    dispatch('timeSelected', selectedDate);
  };

  const selectDateAndTime = () => {
    const newSelect = new Date(`${year}-${month + 1}-${day}:${time}`);
    dispatch('timeSelected', newSelect);
  };

  const handleTimeChange = (data: CustomEvent) => (time = data.detail.time);
</script>

<div class="bg-base-100 absolute top-0 w-80 container">
  <div class={`datePicker bg-base-100 ${showDataAndTimePicker && 'active'}`}>
    <div data-value={selectedDate} class="selectedDate">
      {dateFormatted}
    </div>
    <div on:click={event => event.stopPropagation()} class="dates bg-base-100">
      <div class="month">
        <div on:click={goToPrevMonth} class="arrows">&lt;</div>
        <div class="">
          {calTitle}
        </div>
        <div on:click={goToNextMonth} class="arrows">&gt;</div>
      </div>

      <div class="days">
        {#each days as day, index}
          <div on:click={() => selectDate(index)} class={day.classes}>
            {index + 1}
          </div>
        {/each}
      </div>
      <TimePicker on:timeChanged={handleTimeChange} />
      <button on:click={selectDateAndTime} class="btn btn-xs btn-primary w-full"
        >Select</button
      >
    </div>
  </div>
</div>

<style>
  .container {
    right: 50%;
  }

  .datePicker {
    display: none;
    position: relative;
    width: 100%;
    border-bottom: 2px solid rgb(39, 39, 39);
    cursor: pointer;
    user-select: none;
    color: hsl(0, 0%, 100%);
  }
  .datePicker.active {
    display: block;
  }

  .datePicker .selectedDate {
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    color: hsl(0, 0%, 100%);
    font-size: 20px;
  }

  .datePicker .dates {
    position: absolute;
    top: 100%;
    left: 0;
    right: 0;
  }

  .datePicker .dates .month {
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 2px solid hsl(0, 0%, 15%);
  }

  .datePicker .dates .month .arrows {
    width: 35px;
    display: flex;
    justify-content: center;
    align-items: center;
    color: hsl(0, 0%, 100%);
    font-size: 20px;
  }

  .datePicker .dates .month .arrows:hover {
    background-color: hsl(0, 0%, 19%);
  }

  .datePicker .dates .month .arrows:active {
    background-color: #ca710c;
  }

  .datePicker .dates .days {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    height: 200px;
    border-bottom: 2px solid hsl(0, 0%, 15%);
  }
  .datePicker .dates .days .day {
    display: flex;
    justify-content: center;
    align-items: center;
    color: hsl(0, 0%, 100%);
  }
  .datePicker .dates .days .day.selected {
    background-color: #ca710c;
  }
  .datePicker .dates .days .day:hover:not(.selected) {
    background-color: hsl(0, 0%, 19%);
  }
</style>
