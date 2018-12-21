workflow "Test" {
  on = "push"
  resolves = ["Clippy"]
}

action "onbjerg/actions/cargo@master" {
  uses = "onbjerg/actions/cargo@master"
  args = "test"
}

action "Clippy" {
  uses = "onbjerg/actions/clippy@master"
  needs = ["onbjerg/actions/cargo@master"]
}
