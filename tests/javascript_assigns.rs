use {
    codesort::*,
    std::fmt::Write,
};

#[test]
fn test_javascript_assigns() {
    static INPUT: &str = r#"
        miaou(function(notif, chat, gui, horn, locals, md, prefs, watch, ws){

            var	notifications = [], // array of {r:roomId, rname:roomname, mid:messageid}
                notifMessage, // an object created with md.notificationMessage displaying notifications
                hasWatchUnseen = false,
                nbUnseenMessages = 0,
                lastUserAction = 0; // ms

            notif.updatePingsList = function(){
                if (!vis()) notif.updateTab(!!notifications.length, nbUnseenMessages);
                if (!notifications.length) {
                    if (notifMessage) notifMessage.$md.slideUp($.fn.remove);
                    notifMessage = null;
                    watch.setPings([]);
                    return;
                }
                if (notifMessage) notifMessage.remove();
                var	localPings = [], otherRooms = {};
                notifications.forEach(function(n){
                    if (locals.room.id==n.r) {
                        localPings.push(n);
                    } else {
                        otherRooms[n.r] = n.rname;
                    }
                });
                notifMessage = md.notificationMessage(function($c){
                    if (localPings.length) {
                        $('<div>').addClass('pingroom').append(
                            $('<span>').text(
                                localPings.length +
                                (localPings.length>1 ? ' pings' : ' ping') + ' in this room.'
                            )
                        ).append(
                            $('<button>').addClass('nextping').text("Next ping").click(function(){
                                notif.nextPing();
                                notif.updatePingsList();
                            })
                        ).append(
                            $('<button>').addClass('clearpings').text('clear').click(function(){
                                notif.clearPings(locals.room.id);
                            })
                        ).appendTo($c)
                    }
                    var	otherRoomIds = Object.keys(otherRooms),
                        nbotherrooms = otherRoomIds.length;
                    if (nbotherrooms) {
                        var t = "You've been pinged in room";
                        if (nbotherrooms>1) t += 's';
                        var $otherrooms = $('<div>').append($('<span>').text(t)).appendTo($c);
                        $.each(otherRooms, function(r, rname){
                            var $brs = $('<div>').addClass('pingroom').appendTo($otherrooms);
                            $('<button>').addClass('openroom').text(rname).click(function(){
                                ws.emit('watch_raz');
                                setTimeout(function(){	location = r; }, 250); // timeout so that the raz is sent
                            }).appendTo($brs);
                            $('<button>').addClass('clearpings').text('clear').click(function(){
                                notif.clearPings(r);
                            }).appendTo($brs);
                        });
                        watch.setPings(otherRoomIds);
                    }
                });
            }


            // called in case of user action proving he's right in front of the chat so
            //  we should not ping him
            // If the user action is related to a message, its mid is passed
            notif.userAct = function(mid){
                lastUserAction = Date.now();
                // we assume the user sees the most recent messages if he acts
                $('#messages .message:gt(-10)').each(function(){
                    notif.removePing($(this).attr('mid'), true, true);
                });
                notif.removePing(mid, true, true);
            }

            // goes to next ping in the room. Return true if there's still another one after that
            notif.nextPing = function(){
                lastUserAction = Date.now();
                var done = false;
                for (var i=0; i<notifications.length; i++) {
                    if (notifications[i].r==locals.room.id) {
                        if (done) {
                            return true;
                        } else {
                            md.focusMessage(notifications[i].mid);
                            ws.emit("rm_ping", notifications[i].mid);
                            notifications.splice(i++, 1);
                            done = true;
                        }
                    }
                }
                return false;
            }

            notif.clearPings = function(roomId){
                let mids = [];
                for (var i=notifications.length; i--;) {
                    if (!roomId || notifications[i].r==roomId) {
                        mids.push(notifications[i].mid);
                        notifications.splice(i, 1);
                    }
                }
                if (!mids.length) return;
                ws.emit('rm_pings', mids);
                notif.updatePingsList();
            }

            // tells whether there's a ping related to that room
            notif.hasPing = function(roomId){
                return !!lastNotificationInRoom(roomId);
            }

            // add pings to the list and update the GUI
            notif.pings = function(pings){
                var	changed = false,
                    visible = vis(),
                    lastUserActionAge = Date.now()-lastUserAction,
                    map = notifications.reduce(function(map, n){ map[n.mid]=1;return map; }, {});
                pings.forEach(function(ping){
                    if (map[ping.mid]) return;
                    if (ping.r===locals.room.id && lastUserActionAge < 15000) {
                        ws.emit("rm_ping", ping.mid);
                        return;
                    }
                    notifications.push(ping);
                    changed = true;
                    if (
                        prefs.get("notif")!=="never"
                        && (!visible || prefs.get("nifvis")==="yes")
                    ) {
                        horn.show(ping.mid, ping.rname, ping.authorname, ping.content);
                    }
                });
                notifications.sort(function(a, b){ return a.mid-b.mid });
                if (changed) notif.updatePingsList();
            }

            // called by the server or (most often) in case of any action on a message
            //  (so this is very frequently called on non pings)
            notif.removePing = function(mid, forwardToServer, flash){
                if (!mid) return;
                // we assume here there's at most one notification to a given message
                for (var i=0; i<notifications.length; i++) {
                    if (notifications[i].mid==mid) {
                        if (flash) {
                            var $md = $('#messages .message[mid='+mid+']');
                            if ($md.length) {
                                md.goToMessageDiv($md);
                            }
                        }
                        if (forwardToServer) ws.emit("rm_ping", mid);
                        notifications.splice(i, 1);
                        notif.updatePingsList();
                        return;
                    }
                }
            }

            notif.removePings = function(mids){
                mids.forEach(mid => notif.removePing(mid));
            }

            notif.init = function(){
                $(window).on('focus', onfocus);
                vis(function(){
                    if (vis()) onfocus();
                });
            }
        });
    "#;

    static OUTPUT: &str = r#"
        miaou(function(notif, chat, gui, horn, locals, md, prefs, watch, ws){

            var	notifications = [], // array of {r:roomId, rname:roomname, mid:messageid}
                notifMessage, // an object created with md.notificationMessage displaying notifications
                hasWatchUnseen = false,
                nbUnseenMessages = 0,
                lastUserAction = 0; // ms

            notif.clearPings = function(roomId){
                let mids = [];
                for (var i=notifications.length; i--;) {
                    if (!roomId || notifications[i].r==roomId) {
                        mids.push(notifications[i].mid);
                        notifications.splice(i, 1);
                    }
                }
                if (!mids.length) return;
                ws.emit('rm_pings', mids);
                notif.updatePingsList();
            }

            // tells whether there's a ping related to that room
            notif.hasPing = function(roomId){
                return !!lastNotificationInRoom(roomId);
            }

            notif.init = function(){
                $(window).on('focus', onfocus);
                vis(function(){
                    if (vis()) onfocus();
                });
            }

            // goes to next ping in the room. Return true if there's still another one after that
            notif.nextPing = function(){
                lastUserAction = Date.now();
                var done = false;
                for (var i=0; i<notifications.length; i++) {
                    if (notifications[i].r==locals.room.id) {
                        if (done) {
                            return true;
                        } else {
                            md.focusMessage(notifications[i].mid);
                            ws.emit("rm_ping", notifications[i].mid);
                            notifications.splice(i++, 1);
                            done = true;
                        }
                    }
                }
                return false;
            }

            // add pings to the list and update the GUI
            notif.pings = function(pings){
                var	changed = false,
                    visible = vis(),
                    lastUserActionAge = Date.now()-lastUserAction,
                    map = notifications.reduce(function(map, n){ map[n.mid]=1;return map; }, {});
                pings.forEach(function(ping){
                    if (map[ping.mid]) return;
                    if (ping.r===locals.room.id && lastUserActionAge < 15000) {
                        ws.emit("rm_ping", ping.mid);
                        return;
                    }
                    notifications.push(ping);
                    changed = true;
                    if (
                        prefs.get("notif")!=="never"
                        && (!visible || prefs.get("nifvis")==="yes")
                    ) {
                        horn.show(ping.mid, ping.rname, ping.authorname, ping.content);
                    }
                });
                notifications.sort(function(a, b){ return a.mid-b.mid });
                if (changed) notif.updatePingsList();
            }

            // called by the server or (most often) in case of any action on a message
            //  (so this is very frequently called on non pings)
            notif.removePing = function(mid, forwardToServer, flash){
                if (!mid) return;
                // we assume here there's at most one notification to a given message
                for (var i=0; i<notifications.length; i++) {
                    if (notifications[i].mid==mid) {
                        if (flash) {
                            var $md = $('#messages .message[mid='+mid+']');
                            if ($md.length) {
                                md.goToMessageDiv($md);
                            }
                        }
                        if (forwardToServer) ws.emit("rm_ping", mid);
                        notifications.splice(i, 1);
                        notif.updatePingsList();
                        return;
                    }
                }
            }

            notif.removePings = function(mids){
                mids.forEach(mid => notif.removePing(mid));
            }

            notif.updatePingsList = function(){
                if (!vis()) notif.updateTab(!!notifications.length, nbUnseenMessages);
                if (!notifications.length) {
                    if (notifMessage) notifMessage.$md.slideUp($.fn.remove);
                    notifMessage = null;
                    watch.setPings([]);
                    return;
                }
                if (notifMessage) notifMessage.remove();
                var	localPings = [], otherRooms = {};
                notifications.forEach(function(n){
                    if (locals.room.id==n.r) {
                        localPings.push(n);
                    } else {
                        otherRooms[n.r] = n.rname;
                    }
                });
                notifMessage = md.notificationMessage(function($c){
                    if (localPings.length) {
                        $('<div>').addClass('pingroom').append(
                            $('<span>').text(
                                localPings.length +
                                (localPings.length>1 ? ' pings' : ' ping') + ' in this room.'
                            )
                        ).append(
                            $('<button>').addClass('nextping').text("Next ping").click(function(){
                                notif.nextPing();
                                notif.updatePingsList();
                            })
                        ).append(
                            $('<button>').addClass('clearpings').text('clear').click(function(){
                                notif.clearPings(locals.room.id);
                            })
                        ).appendTo($c)
                    }
                    var	otherRoomIds = Object.keys(otherRooms),
                        nbotherrooms = otherRoomIds.length;
                    if (nbotherrooms) {
                        var t = "You've been pinged in room";
                        if (nbotherrooms>1) t += 's';
                        var $otherrooms = $('<div>').append($('<span>').text(t)).appendTo($c);
                        $.each(otherRooms, function(r, rname){
                            var $brs = $('<div>').addClass('pingroom').appendTo($otherrooms);
                            $('<button>').addClass('openroom').text(rname).click(function(){
                                ws.emit('watch_raz');
                                setTimeout(function(){	location = r; }, 250); // timeout so that the raz is sent
                            }).appendTo($brs);
                            $('<button>').addClass('clearpings').text('clear').click(function(){
                                notif.clearPings(r);
                            }).appendTo($brs);
                        });
                        watch.setPings(otherRoomIds);
                    }
                });
            }


            // called in case of user action proving he's right in front of the chat so
            //  we should not ping him
            // If the user action is related to a message, its mid is passed
            notif.userAct = function(mid){
                lastUserAction = Date.now();
                // we assume the user sees the most recent messages if he acts
                $('#messages .message:gt(-10)').each(function(){
                    notif.removePing($(this).attr('mid'), true, true);
                });
                notif.removePing(mid, true, true);
            }
        });
    "#;

    let list = List::from_str(INPUT, Language::JavaScript).unwrap();
    list.tprint();
    let range = LineNumberRange {
        start: LineNumber::new(9).unwrap(),
        end: LineNumber::new(171).unwrap(),
    };
    let window = list.window_on_line_range(range).unwrap();

    let mut output = String::new();
    write!(output, "{}", window.sort().unwrap()).unwrap();
    println!("{}", output);
    assert_eq!(output, OUTPUT);
}
