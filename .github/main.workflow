workflow "Test" {
  on = "push"
  resolves = ["onbjerg/actions/cargo@master"]
}

action "onbjerg/actions/cargo@master" {
  uses = "onbjerg/actions/cargo@master"
  args = "test"
}
