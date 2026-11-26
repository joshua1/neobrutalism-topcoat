//! The hero: the headline, the calls to action and a task board that works.
//!
//! The board is rendered on the server and kept in the DOM afterwards: cards
//! can be dragged between columns, the arrow on a card moves it to the next
//! column, "New issue" adds a card, and the activity line says what happened.
//! A small inline script ([`BOARD_SCRIPT`]) does this, because moving
//! elements between lists and drag and drop are outside the runtime's
//! expressions. Like the original, dragging uses the HTML drag and drop API,
//! which touch screens mostly don't fire; the arrow buttons work everywhere.

use topcoat::{
    Result,
    context::Cx,
    icon::{icon, iconify::iconify_icon},
    view::{Unescaped, View, attributes, class, component, view},
};

use super::decor_star::{GRID_BACKGROUND, section_star};
use crate::components::{
    button::{ButtonVariant, button},
    stars::Star,
};

/// A board column, in order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Status {
    Todo,
    Progress,
    Done,
}

impl Status {
    /// The column's `data-column`, also used by [`BOARD_SCRIPT`].
    fn key(self) -> &'static str {
        match self {
            Self::Todo => "todo",
            Self::Progress => "progress",
            Self::Done => "done",
        }
    }

    fn title(self) -> &'static str {
        match self {
            Self::Todo => "Todo",
            Self::Progress => "In progress",
            Self::Done => "Done",
        }
    }

    /// Where a card's arrow button sends it: done cards go back to the start.
    fn next(self) -> Self {
        match self {
            Self::Todo => Self::Progress,
            Self::Progress => Self::Done,
            Self::Done => Self::Todo,
        }
    }
}

const COLUMNS: [Status; 3] = [Status::Todo, Status::Progress, Status::Done];

struct Task {
    id: &'static str,
    title: &'static str,
    tag: &'static str,
    tag_color: &'static str,
    status: Status,
    /// Whether an agent is working on it.
    agent: bool,
}

const INITIAL_TASKS: [Task; 6] = [
    Task {
        id: "PLN-142",
        title: "Onboarding checklist v2",
        tag: "Design",
        tag_color: "bg-chart-5",
        status: Status::Todo,
        agent: false,
    },
    Task {
        id: "PLN-139",
        title: "Rate limit the public API",
        tag: "Backend",
        tag_color: "bg-chart-2",
        status: Status::Todo,
        agent: false,
    },
    Task {
        id: "PLN-131",
        title: "Usage-based billing",
        tag: "Billing",
        tag_color: "bg-chart-4",
        status: Status::Progress,
        agent: true,
    },
    Task {
        id: "PLN-128",
        title: "Fix flaky e2e tests",
        tag: "Infra",
        tag_color: "bg-chart-3",
        status: Status::Progress,
        agent: false,
    },
    Task {
        id: "PLN-120",
        title: "Dark mode for dashboard",
        tag: "Design",
        tag_color: "bg-chart-5",
        status: Status::Done,
        agent: false,
    },
    Task {
        id: "PLN-117",
        title: "Slack notifications",
        tag: "Integrations",
        tag_color: "bg-chart-1",
        status: Status::Done,
        agent: false,
    },
];

/// The empty card the script clones for a new issue.
const BLANK_TASK: Task = Task {
    id: "",
    title: "",
    tag: "",
    tag_color: "",
    status: Status::Todo,
    agent: false,
};

/// Moves, adds and drags cards, keeping the counts, the arrow buttons' labels
/// and the activity line in step. The board element is the script's previous
/// sibling. New issues cycle through `issues`, numbered from PLN-143.
const BOARD_SCRIPT: &str = r#"(() => {
  const board = document.currentScript.previousElementSibling;
  const titles = { todo: "Todo", progress: "In progress", done: "Done" };
  const next = { todo: "progress", progress: "done", done: "todo" };
  const issues = [
    ["Write release notes", "Docs", "bg-chart-3"],
    ["Audit onboarding emails", "Growth", "bg-chart-4"],
    ["Fix avatar upload", "Bug", "bg-chart-2"],
    ["Add SSO to admin panel", "Security", "bg-chart-5"],
  ];
  let number = 143;
  let dragging = null;
  const columns = () => board.querySelectorAll("[data-column]");
  const at = (e) => (e.target instanceof Element ? e.target : e.target.parentElement);
  const say = (actor, text) => {
    board.querySelector("[data-actor]").textContent = actor;
    board.querySelector("[data-activity]").textContent = text;
  };
  const refresh = () => {
    for (const column of columns()) {
      const cards = column.querySelectorAll("[data-task]");
      column.querySelector("[data-count]").textContent = cards.length;
      const to = titles[next[column.dataset.column]];
      for (const card of cards) {
        card.querySelector("[data-move]").setAttribute("aria-label", `Move ${card.dataset.task} to ${to}`);
      }
    }
  };
  const place = (card, status) => {
    const column = board.querySelector(`[data-column="${status}"]`);
    column.insertBefore(card, column.querySelector("[data-add]"));
    refresh();
  };
  const move = (id, to) => {
    const card = board.querySelector(`[data-task="${CSS.escape(id)}"]`);
    if (!card) return;
    if (card.closest("[data-column]").dataset.column !== to) place(card, to);
    say("You", `moved ${id} to ${titles[to]}`);
  };
  const add = () => {
    const id = `PLN-${number}`;
    const [title, tag, color] = issues[number % issues.length];
    number += 1;
    const card = board.querySelector("template").content.firstElementChild.cloneNode(true);
    card.dataset.task = id;
    card.querySelector("[data-id]").textContent = id;
    card.querySelector("[data-title]").textContent = title;
    const label = card.querySelector("[data-tag]");
    label.textContent = tag;
    label.classList.add(color);
    place(card, "todo");
    say("You", `created ${id}`);
  };
  const highlight = (target) => {
    for (const column of columns()) {
      column.toggleAttribute("data-over", column === target && dragging !== null);
    }
  };
  const end = () => {
    dragging?.removeAttribute("data-dragging");
    dragging = null;
    highlight(null);
  };
  board.addEventListener("click", (e) => {
    const target = at(e);
    const button = target.closest("[data-move]");
    if (button) {
      const card = button.closest("[data-task]");
      move(card.dataset.task, next[card.closest("[data-column]").dataset.column]);
    } else if (target.closest("[data-add]")) {
      add();
    }
  });
  board.addEventListener("dragstart", (e) => {
    const card = at(e).closest("[data-task]");
    if (!card) return;
    e.dataTransfer.setData("text/plain", card.dataset.task);
    e.dataTransfer.effectAllowed = "move";
    dragging = card;
    card.setAttribute("data-dragging", "");
  });
  board.addEventListener("dragend", end);
  board.addEventListener("dragover", (e) => {
    const column = at(e).closest("[data-column]");
    if (!column) return;
    e.preventDefault();
    highlight(column);
  });
  board.addEventListener("dragleave", (e) => {
    const column = at(e).closest("[data-column]");
    if (column && !column.contains(e.relatedTarget)) column.removeAttribute("data-over");
  });
  board.addEventListener("drop", (e) => {
    const column = at(e).closest("[data-column]");
    if (!column) return;
    e.preventDefault();
    const id = e.dataTransfer.getData("text/plain");
    if (id) move(id, column.dataset.column);
    end();
  });
})();"#;

/// A draggable card. Its column (`group/column`) decides the arrow's icon and
/// whether the title is struck through, so moving the card restyles it.
#[component]
async fn task_card(cx: &Cx, task: &Task) -> Result<impl View> {
    Ok(view! {
        <div
            data-task=(task.id)
            draggable="true"
            class="cursor-grab rounded-base border-2 border-border bg-secondary-background p-3 shadow-[2px_2px_0_0_var(--border)] active:cursor-grabbing data-dragging:border-dashed data-dragging:shadow-none"
        >
            <div class="flex items-center justify-between text-[11px] font-base">
                <span class="flex items-center gap-1.5">
                    <span data-id="">(task.id)</span>
                    if task.agent {
                        icon(data: iconify_icon!("lucide:bot"), attrs: attributes! { class="size-3.5" })
                    }
                </span>
                <button
                    type="button"
                    data-move=""
                    aria-label=(format!("Move {} to {}", task.id, task.status.next().title()))
                    class="flex size-6 items-center justify-center rounded-base border-2 border-border bg-main text-main-foreground transition-transform hover:translate-x-px hover:translate-y-px"
                >
                    icon(
                        data: iconify_icon!("lucide:arrow-right"),
                        attrs: attributes! { class="size-3 group-data-[column=done]/column:hidden" }
                    )
                    icon(
                        data: iconify_icon!("lucide:rotate-ccw"),
                        attrs: attributes! { class="hidden size-3 group-data-[column=done]/column:block" }
                    )
                </button>
            </div>
            <p data-title="" class="mt-1 text-sm font-heading leading-snug group-data-[column=done]/column:line-through">
                (task.title)
            </p>
            <span
                data-tag=""
                class=(class!(
                    "mt-2 inline-block rounded-base border-2 border-border px-1.5 text-[10px] font-heading",
                    task.tag_color,
                ))
            >
                (task.tag)
            </span>
        </div>
    })
}

#[component]
async fn task_board(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        <div class="relative mx-auto w-full max-w-5xl">
            <div class="absolute -right-2 -top-5 z-10 flex rotate-3 items-center gap-1.5 rounded-base border-2 border-border bg-chart-3 px-3 py-1.5 text-sm font-heading shadow-shadow">
                icon(data: iconify_icon!("lucide:trending-down"), attrs: attributes! { class="size-4" })
                " Cycle time −32%"
            </div>

            <div class="overflow-hidden rounded-base border-2 border-border bg-background shadow-shadow">
                <div class="flex items-center justify-between border-b-2 border-border bg-background px-4 py-2.5">
                    <div class="flex flex-1 items-center gap-1.5">
                        <span class="size-3 rounded-full border-2 border-border bg-chart-2"></span>
                        <span class="size-3 rounded-full border-2 border-border bg-chart-3"></span>
                        <span class="size-3 rounded-full border-2 border-border bg-chart-4"></span>
                        <span class="ml-3 hidden text-xs font-base md:inline">"Drag cards to move them"</span>
                    </div>
                    <span class="shrink-0 text-xs font-heading">"Sprint 24 · Planly"</span>
                    <span class="flex-1"></span>
                </div>

                <div class="grid auto-cols-[78%] grid-flow-col gap-3 overflow-x-auto p-4 pb-12 sm:auto-cols-auto sm:grid-flow-row sm:grid-cols-3 sm:gap-5 sm:overflow-visible sm:p-6 sm:pb-12">
                    #[key(column.key())]
                    for column in COLUMNS {
                        let tasks: Vec<&Task> =
                            INITIAL_TASKS.iter().filter(|task| task.status == column).collect();
                        <div
                            data-column=(column.key())
                            class="group/column -m-2 flex flex-col gap-3 rounded-base border-2 border-transparent p-2 transition-colors data-over:border-dashed data-over:border-border data-over:bg-secondary-background"
                        >
                            <div class="flex items-center justify-between text-xs font-heading">
                                (column.title())
                                <span data-count="" class="rounded-base border-2 border-border bg-secondary-background px-1.5">
                                    (tasks.len())
                                </span>
                            </div>

                            #[key(task.id)]
                            for task in tasks {
                                task_card(task: task)
                            }

                            if column == Status::Todo {
                                <button
                                    type="button"
                                    data-add=""
                                    class="flex items-center justify-center gap-1.5 rounded-base border-2 border-dashed border-border px-3 py-2 text-xs font-heading transition-colors hover:bg-main hover:text-main-foreground"
                                >
                                    icon(data: iconify_icon!("lucide:plus"), attrs: attributes! { class="size-3.5" })
                                    " New issue"
                                </button>
                            }
                        </div>
                    }
                </div>
            </div>

            <div
                aria-live="polite"
                class="absolute -bottom-6 -left-2 z-10 flex -rotate-3 items-center gap-2 rounded-base border-2 border-border bg-secondary-background px-3 py-2 text-sm font-base shadow-shadow sm:-left-6"
            >
                <span class="flex size-6 items-center justify-center rounded-base border-2 border-border bg-chart-3">
                    icon(data: iconify_icon!("lucide:sparkles"), attrs: attributes! { class="size-3.5" })
                </span>
                <span>
                    <span data-actor="" class="font-heading">"Agent"</span>
                    " "
                    <span data-activity="">"moved PLN-131 to In progress"</span>
                </span>
            </div>

            <template>task_card(task: &BLANK_TASK)</template>
        </div>
        <script>(Unescaped::new_unchecked(BOARD_SCRIPT))</script>
    })
}

#[component]
pub async fn hero(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        <section class=(class!(
            "relative isolate overflow-hidden border-b-4 border-border bg-secondary-background",
            GRID_BACKGROUND,
        ))>
            section_star(
                shape: Star::S14,
                color: "var(--chart-3)",
                class: "-left-16 lg:-left-12 top-8 size-24 rotate-[15deg] lg:size-40",
            )
            section_star(
                shape: Star::S26,
                color: "var(--chart-4)",
                class: "-right-16 lg:-right-12 top-[30%] size-24 -rotate-[40deg] lg:size-40",
            )
            <div class="mx-auto flex w-full max-w-6xl flex-col items-center gap-16 px-5 py-20 lg:py-24">
                <div class="flex flex-col items-center gap-7 text-center">
                    <h1 class="font-heading text-4xl leading-[1.05] tracking-tight sm:text-5xl lg:text-6xl">
                        "Plan less."
                        <br>
                        "Ship "
                        <span class="inline-block -rotate-2 rounded-base border-2 border-border bg-chart-3 px-3 shadow-shadow">
                            "more."
                        </span>
                    </h1>

                    <p class="max-w-xl text-lg font-base">
                        "Planly is the project tracker for teams that move fast. Issues, roadmaps and AI triage in one fast, focused workspace."
                    </p>

                    <div class="flex flex-wrap justify-center gap-3 sm:gap-4">
                        button(attrs: attributes! { class="sm:h-11 sm:px-8" }, "Start free trial")
                        button(
                            variant: ButtonVariant::Neutral,
                            attrs: attributes! { class="sm:h-11 sm:px-8" },
                            "Book a demo"
                        )
                    </div>
                </div>

                task_board()
            </div>
        </section>
    })
}
