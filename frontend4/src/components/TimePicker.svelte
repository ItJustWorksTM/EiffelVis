<script lang="ts">
  import { onMount, createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();
  const currentTime = new Date();
  let hour = currentTime.getHours();
  let minute = currentTime.getMinutes();
  let second = currentTime.getSeconds();

  const formatTime = (t: number) => {
    let formatted: number | string = t;
    if (t < 10) {
      formatted = `0${t}`;
    }
    return formatted;
  };

  $: time = `${formatTime(hour)}:${formatTime(minute)}:${formatTime(second)}`;

  onMount(() => {
    dispatch('timeChanged', { time });
  });

  $: dispatch('timeChanged', { time });

  const hourChange = e => {
    if (+e.target.value > 23) {
      hour = 23;
    } else if (+e.target.value < 0) {
      hour = 0;
    } else if (e.target.value === '') {
      hour = 0;
    } else {
      hour = +e.target.value;
    }
  };

  const minuteChange = e => {
    if (+e.target.value > 59) {
      minute = 59;
    } else if (+e.target.value < 0) {
      minute = 0;
    } else if (e.target.value === '') {
      minute = 0;
    } else {
      minute = +e.target.value;
    }
  };

  const secondChange = e => {
    if (+e.target.value > 59) {
      second = 59;
    } else if (+e.target.value < 0) {
      second = 0;
    } else if (e.target.value === '') {
      second = 0;
    } else {
      second = +e.target.value;
    }
  };

  const hourUp = () => {
    hour++;
    if (hour > 23) {
      hour = 0;
    }
    time = `${formatTime(hour)}:${formatTime(minute)}:${formatTime(second)}`;
  };
  const hourDown = () => {
    hour--;
    if (hour < 0) {
      hour = 23;
    }
  };

  const minuteUp = () => {
    minute++;
    if (minute > 59) {
      minute = 0;
      hourUp();
    }
  };
  const minuteDown = () => {
    minute--;
    if (minute < 0) {
      minute = 59;
      hourDown();
    }
  };

  const secondUp = () => {
    second++;
    if (second > 59) {
      second = 0;
      minuteUp();
    }
  };
  const secondDown = () => {
    second--;
    if (second < 0) {
      second = 59;
      minuteDown();
    }
  };
</script>

<div class="timePicker" data-time={time}>
  <div class="hour">
    <div role="textbox" on:click={hourUp} class="hrUp" />
    <input
      placeholder="00"
      type="number"
      class="hr"
      bind:value={hour}
      onChange={hourChange}
    />
    <div on:click={hourDown} class="hrDown" />
  </div>

  <div class="separator">:</div>

  <div class="minute">
    <div on:click={minuteUp} class="minUp" />
    <input
      placeholder="00"
      type="number"
      class="min"
      value={minute}
      onChange={minuteChange}
    />
    <div on:click={minuteDown} class="minDown" />
  </div>

  <div class="separator">:</div>

  <div class="second">
    <div role="textbox" on:click={secondUp} class="secUp" />
    <input
      placeholder="00"
      type="number"
      class="sec"
      value={second}
      onChange={secondChange}
    />
    <div role="textbox" on:click={secondDown} class="secDown" />
  </div>
</div>

<style>
  input[type='number']::-webkit-outer-spin-button,
  input[type='number']::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
    color: hsl(0, 0%, 100%);
  }

  input[type='number'] {
    -moz-appearance: textfield;
    color: hsl(0, 0%, 100%);
  }

  .timePicker {
    display: flex;
    justify-content: space-around;
    align-items: center;
    width: 100%;
    max-width: 200px;
    margin: 0 auto;
    padding: 25px 15px;
    background-color: transparent;
    color: hsl(0, 0%, 100%);
    font-size: 20px;
    font-weight: 700;
    user-select: none;
  }

  .timePicker .hour,
  .timePicker .minute,
  .timePicker .second {
    position: relative;
    min-width: 60px;
    text-align: center;
    display: flex;
    justify-content: stretch;
    align-items: stretch;
  }

  .timePicker .hour .hr,
  .timePicker .minute .min,
  .timePicker .second .sec {
    background: none;
    font-size: 20px;
    appearance: none;
    border: none;
    outline: none;
    display: block;
    width: 100%;
    text-align: center;
  }

  .timePicker .hour .hrUp,
  .timePicker .hour .hrDown,
  .timePicker .minute .minUp,
  .timePicker .minute .minDown,
  .timePicker .second .secUp,
  .timePicker .second .secDown {
    position: absolute;
    left: 50%;
    transform: translateX(-50%);
    width: 15px;
    height: 20px;
    border-left: 15px solid transparent;
    border-right: 15px solid transparent;
    cursor: pointer;
  }

  .timePicker .hour .hrUp,
  .timePicker .minute .minUp,
  .timePicker .second .secUp {
    top: -15px;
    border-bottom: 15px solid hsl(0, 0%, 100%);
  }

  .timePicker .hour .hrDown,
  .timePicker .minute .minDown,
  .timePicker .second .secDown {
    bottom: -17px;
    border-top: 15px solid hsl(0, 0%, 100%);
  }

  .timePicker .hour .hrUp:hover,
  .timePicker .hour .hrDown:hover,
  .timePicker .minute .minUp:hover,
  .timePicker .minute .minDown:hover,
  .timePicker .second .secUp:hover,
  .timePicker .second .secDown:hover {
    border-bottom-color: #53565a;
    border-top-color: #53565a;
  }
</style>
