"use client";
import { TrendingUp } from "lucide-react";
import { Label, PolarRadiusAxis, RadialBar, RadialBarChart } from "recharts";

import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  ChartConfig,
  ChartContainer,
  ChartTooltip,
  ChartTooltipContent,
} from "@/components/ui/chart";
import { useTasks } from "@/context/taskContext";

export const description = "A radial chart with stacked sections";

// SET LABEL AND COLOR CHART
const chartConfig = {
  desktop: {
    label: "Completed",
    color: "#8BCE89",
  },
  mobile: {
    label: "Pending",
    color: "#EB4E31",
  },
} satisfies ChartConfig;

function RadialCHart() {
  const { tasks, completedTasks, activeTasks } = useTasks();
  const tasksTotal = tasks?.length;
  const activeTasksTotal = activeTasks?.length;
  const completedTasksTotal = completedTasks?.length;

  let done = true;
  if (tasksTotal === 0 && completedTasksTotal >= 1) {
    done = true;
  } else {
    done = false;
  }

  const chartData = [
    {
      pending: activeTasksTotal,
      completed: completedTasksTotal,
    },
  ];

  return (
    <Card className="flex flex-col border-2 border-white shadow-none bg-[#000D0D]">
      <CardHeader className="items-center pb-0">
        <CardTitle>Comleted vs Pending Tasks</CardTitle>
        <CardDescription>Task completion status.</CardDescription>
      </CardHeader>
      <CardContent className="flex flex-1 items-center pb-0">
        <ChartContainer
          config={chartConfig}
          className="mx-auto aspect-square w-full max-w-[250px]"
        >
          <RadialBarChart
            data={chartData}
            endAngle={180}
            innerRadius={80}
            outerRadius={130}
          >
            <ChartTooltip
              cursor={false}
              content={<ChartTooltipContent hideLabel />}
            />
            <PolarRadiusAxis tick={false} tickLine={false} axisLine={false}>
              <Label
                content={({ viewBox }) => {
                  if (viewBox && "cx" in viewBox && "cy" in viewBox) {
                    return (
                      <text x={viewBox.cx} y={viewBox.cy} textAnchor="middle">
                        <tspan
                          x={viewBox.cx}
                          y={(viewBox.cy || 0) - 16}
                          className="fill-white text-2xl font-bold"
                          // style={{ fill: "red" }}
                        >
                          {tasksTotal}
                        </tspan>
                        <tspan
                          x={viewBox.cx}
                          y={(viewBox.cy || 0) + 4}
                          className="fill-gray-400"
                        >
                          Tasks
                        </tspan>
                      </text>
                    );
                  }
                }}
              />
            </PolarRadiusAxis>
            <RadialBar
              dataKey="completed"
              stackId="a"
              cornerRadius={5}
              fill="var(--color-desktop)"
              className="stroke-transparent stroke-2"
            />
            <RadialBar
              dataKey="pending"
              fill="var(--color-mobile)"
              stackId="a"
              cornerRadius={5}
              className="stroke-transparent stroke-2"
            />
          </RadialBarChart>
        </ChartContainer>
      </CardContent>
      {/* <CardFooter className="flex-col gap-2 text-sm">
        <div className="flex items-center gap-2 font-medium leading-none">
          Task completion improved by 12% this month{" "}
          <TrendingUp className="h-4 w-4" />
        </div>
        <div className="leading-none text-gray-400">
          Analysis based on tasks completed in the last 30 days.
        </div>
      </CardFooter> */}
      <CardFooter className="text-sm text-center">
        <div className="items-center font-medium leading-none">
          {done ? (
            <div className="ml-[0.7rem]">
              <p
                className="mb-3 text-2xl font-bold"
                style={{
                  background:
                    "conic-gradient(from 90deg at 50% 50%, #E2CBFF 0%, #393BB2 50%, #E2CBFF 100%)",
                  WebkitBackgroundClip: "text",
                  WebkitTextFillColor: "transparent",
                  display: "inline-block",
                  backgroundSize: "200% 200%",
                  animation: "rainbow-spin 3s linear infinite",
                }}
              >
                You are amazing!
              </p>
              <p>
                Take a cup of coffee
                <br />
                and kick-back!
              </p>
            </div>
          ) : (
            <div className="ml-[0.7rem]">
              <p className="mb-3 text-xl font-bold text-blue-400">
                You are doing great!
              </p>
              <p>Keep going!</p>
            </div>
          )}
        </div>
      </CardFooter>
    </Card>
  );
}

export default RadialCHart;
