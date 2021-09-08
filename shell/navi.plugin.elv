use str

fn call-navi []{
  if (eq $edit:current-command '') {
    answer = (navi --print)
    edit:replace-input $answer
  } elif (not (str:contains-any $edit:current-command '|')) {
    answer = (navi --print --query $edit:current-command)
    if (not-eq $answer '') {
      edit:replace-input $answer
    }
  } else {
    cmds = [(str:split '|' $edit:current-command)]
    qty = (- (count $cmds) 1)
    query = (all $cmds | drop $qty)
    cmds = [(all $cmds | take $qty)]
    answer = ''
    if (eq $query '') {
      answer = (navi --print)
    } else {
      answer = (navi --print --query $query)
    }

    if (not-eq $answer '') {
      cmds = [$@cmds $answer]
      edit:replace-input (str:join '| ' $cmds)
    }
  }
}

edit:insert:binding[Alt-h] = []{ call-navi >/dev/tty 2>&1 }