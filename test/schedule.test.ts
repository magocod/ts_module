import moment from "moment";

enum ScheduleType {
  academic,
  playtime,
}

interface Schedule {
  start: string;
  end: string;
  type: ScheduleType;
  assignedSchedule: AssignedSchedule | null;
}

interface AssignedSchedule {
  _id: string;
  day: number;
  start: string;
  end: string;
}

describe("schedule", function () {
  it("v1", function () {
    const duration = 45;
    const academicHours = 7;
    const hoursBreaks = 1;

    const schedules: Schedule[] = [];

    const start = moment().set({ hours: 7, minutes: 0, seconds: 0 });
    console.log("start", start.format("hh:mm"));
    start.month();

    const end = moment().set({ hours: 11, minutes: 0, seconds: 0 });
    console.log("end", end.format("hh:mm"));

    for (let i = 0; i < academicHours; i++) {
      const last = start.format("hh:mm");
      start.add(45, "m");
      // console.log("i", start.format("hh:mm"));

      schedules.push({
        start: last,
        end: start.format("hh:mm"),
        type: ScheduleType.academic,
        assignedSchedule: null,
      });

      if (start.isSameOrAfter(end)) {
        // console.log("stop");
        // console.log(start.format("hh:mm"));
        break;
      }
    }
    // console.log(schedules.length);

    // FOR OF
    const r: Schedule = {
      start: "09:15",
      end: "09:45",
      type: ScheduleType.playtime,
      assignedSchedule: null,
    };
    //
    // let update = false;
    // let addIndex = 0;
    //
    // for (const [i, s] of schedules.entries()) {
    //   if (s.start === r.start) {
    //     update = true;
    //     addIndex = i;
    //
    //     const dif = moment.duration(
    //       moment(moment().format("YYYY-MM-DD ") + r.end).diff(moment(moment().format("YYYY-MM-DD ") + r.start))
    //     );
    //     console.log(dif.asMinutes())
    //
    //     s.start = moment(moment().format("YYYY-MM-DD ") + s.start)
    //       .add(dif.asMinutes(), "m")
    //       .format("hh:mm");
    //     s.end = moment(moment().format("YYYY-MM-DD ") + s.end)
    //       .add(dif.asMinutes(), "m")
    //       .format("hh:mm");
    //     continue;
    //   }
    //
    //   if (update) {
    //     s.start = moment(moment().format("YYYY-MM-DD ") + s.start)
    //       .add(45, "m")
    //       .format("hh:mm");
    //     s.end = moment(moment().format("YYYY-MM-DD ") + s.end)
    //       .add(45, "m")
    //       .format("hh:mm");
    //   }
    // }
    //
    // schedules.splice(addIndex, 0, r);

    const rList: Schedule[] = [];
    rList.push(r);
    rList.push({
      start: "01:00",
      end: "01:30",
      type: ScheduleType.playtime,
      assignedSchedule: null,
    });
    rList.push({
      start: "10:45",
      end: "11:00",
      type: ScheduleType.playtime,
      assignedSchedule: null,
    });

    for (const rs of rList) {
      let update = false;
      let addIndex = -1;

      for (const [i, s] of schedules.entries()) {
        const dif = moment.duration(
          moment(moment().format("YYYY-MM-DD ") + rs.end).diff(
            moment(moment().format("YYYY-MM-DD ") + rs.start)
          )
        );

        const last = moment(moment().format("YYYY-MM-DD ") + s.end);

        if (s.start === rs.start) {
          update = true;
          addIndex = i;

          const dif = moment.duration(
            moment(moment().format("YYYY-MM-DD ") + rs.end).diff(
              moment(moment().format("YYYY-MM-DD ") + rs.start)
            )
          );
          console.log(dif.asMinutes());

          last.add(dif.asMinutes(), "m");
          s.start = last.format("hh:mm");
          s.end = moment(moment().format("YYYY-MM-DD ") + rs.end)
            .add(dif.asMinutes(), "m")
            .format("hh:mm");
          continue;
        }

        if (update) {
          s.start = last.add(45, "m").format("hh:mm");
          s.end = moment(moment().format("YYYY-MM-DD ") + s.end)
            .add(45, "m")
            .add(dif.asMinutes(), "m")
            .format("hh:mm");
        }
      }

      if (addIndex !== -1) {
        schedules.splice(addIndex, 0, rs);
      }
    }

    // schedules.sort(function (a, b) {
    //   // Turn your strings into dates, and then subtract them
    //   // to get a value that is either negative, positive, or zero.
    //   return (
    //     new Date(moment().format("YYYY-MM-DD ") + a.start).getTime() -
    //     new Date(moment().format("YYYY-MM-DD ") + b.start).getTime()
    //   );
    // });

    console.log(JSON.stringify(schedules, null, 2));

    const list: AssignedSchedule[] = [
      // A
      {
        _id: "630e2cdf8febfb67f63f8e78",
        day: 1,
        start: "07:00",
        end: "07:45",
      },
      {
        _id: "630e2d8cb58ca1cca5fdf2b9",
        day: 1,
        start: "08:30",
        end: "09:15",
      },
      // B
      {
        _id: "630e2fb2b58ca1cca5fdf347",
        day: 1,
        start: "07:45",
        end: "08:30",
      },
    ];

    for (const e of list) {
      const indexUpdate = schedules.findIndex((sc) => {
        return sc.start === e.start && sc.end === e.end;
      });
      // console.log(indexUpdate)

      if (indexUpdate !== -1) {
        schedules[indexUpdate].assignedSchedule = e;
      }
    }

    console.log(JSON.stringify(schedules, null, 2));

    // const v = hello_js(name);
    // // console.log(v);
    //
    // expect(v).toBe(`${Config.hello_message()} -> ${name}`);
  });

  it("v2", function () {
    const duration = 45;
    const academicHours = 7;
    const hoursBreaks = 1;

    const schedules: Schedule[] = [];

    const start = moment().set({ hours: 7, minutes: 0, seconds: 0 });
    console.log("start", start.format("hh:mm"));
    start.month();

    const end = moment().set({ hours: 11, minutes: 0, seconds: 0 });
    console.log("end", end.format("hh:mm"));

    const r: Schedule = {
      start: "09:15",
      end: "09:45",
      type: ScheduleType.playtime,
      assignedSchedule: null,
    };

    const rList: Schedule[] = [];
    rList.push(r);
    // rList.push({
    //   start: "01:00",
    //   end: "01:30",
    //   type: ScheduleType.playtime,
    //   assignedSchedule: null,
    // });
    rList.push({
      start: "10:45",
      end: "11:00",
      type: ScheduleType.playtime,
      assignedSchedule: null,
    });

    let last = start.format("hh:mm");

    for (let i = 0; i < academicHours; i++) {
      // search recess
      const r = rList.find((v) => {
        const rStart = moment("2022-10-10 " + v.end);
        return start.isSameOrBefore(rStart);
      });
      // console.log(r);

      if (r !== undefined) {
        const dif = moment.duration(
          moment(moment().format("YYYY-MM-DD ") + r.end).diff(
            moment(moment().format("YYYY-MM-DD ") + r.start)
          )
        );
        // console.log(dif.asMinutes());
        start.add(dif.asMinutes(), "m");

        schedules.push({
          start: last,
          end: start.format("hh:mm"),
          type: ScheduleType.academic,
          assignedSchedule: null,
        });

        last = start.format("hh:mm");
      }

      start.add(45, "m");
      // console.log("i", start.format("hh:mm"));

      schedules.push({
        start: last,
        end: start.format("hh:mm"),
        type: ScheduleType.academic,
        assignedSchedule: null,
      });

      if (start.isSameOrAfter(end)) {
        // console.log("stop");
        // console.log(start.format("hh:mm"));
        break;
      }

      last = start.format("hh:mm");
    }
    // console.log(schedules.length);

    console.log(JSON.stringify(schedules, null, 2));

    // const list: AssignedSchedule[] = [
    //   // A
    //   {
    //     _id: "630e2cdf8febfb67f63f8e78",
    //     day: 1,
    //     start: "07:00",
    //     end: "07:45",
    //   },
    //   {
    //     _id: "630e2d8cb58ca1cca5fdf2b9",
    //     day: 1,
    //     start: "08:30",
    //     end: "09:15",
    //   },
    //   // B
    //   {
    //     _id: "630e2fb2b58ca1cca5fdf347",
    //     day: 1,
    //     start: "07:45",
    //     end: "08:30",
    //   },
    // ];
    //
    // for (const e of list) {
    //   const indexUpdate = schedules.findIndex((sc) => {
    //     return sc.start === e.start && sc.end === e.end;
    //   });
    //   // console.log(indexUpdate)
    //
    //   if (indexUpdate !== -1) {
    //     schedules[indexUpdate].assignedSchedule = e;
    //   }
    // }
    //
    // console.log(JSON.stringify(schedules, null, 2));

    // const v = hello_js(name);
    // // console.log(v);
    //
    // expect(v).toBe(`${Config.hello_message()} -> ${name}`);
  });
});
