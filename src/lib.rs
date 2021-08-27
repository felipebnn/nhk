use std::env;
use std::fs::File;
use std::thread;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::time;

use evdev_rs::{ Device, ReadFlag, GrabMode, InputEvent as InInputEvent };

use evdev::{uinput::VirtualDevice, uinput::VirtualDeviceBuilder, AttributeSet, EventType, InputEvent as OutInputEvent, Key, RelativeAxisType };

pub type EventHandler = Box<dyn Fn(InInputEvent, &Sender<OutInputEvent>) -> bool>;

fn dev_uinput_from_file(file_name: String) -> Result<(Device, VirtualDevice), std::io::Error> {
    let file = File::open(file_name).unwrap();
    let dev = Device::new_from_file(file).unwrap();

    let mut keys = AttributeSet::<Key>::new();
    {
        keys.insert(Key::KEY_RESERVED);
        keys.insert(Key::KEY_ESC);
        keys.insert(Key::KEY_1);
        keys.insert(Key::KEY_2);
        keys.insert(Key::KEY_3);
        keys.insert(Key::KEY_4);
        keys.insert(Key::KEY_5);
        keys.insert(Key::KEY_6);
        keys.insert(Key::KEY_7);
        keys.insert(Key::KEY_8);
        keys.insert(Key::KEY_9);
        keys.insert(Key::KEY_0);
        keys.insert(Key::KEY_MINUS);
        keys.insert(Key::KEY_EQUAL);
        keys.insert(Key::KEY_BACKSPACE);
        keys.insert(Key::KEY_TAB);
        keys.insert(Key::KEY_Q);
        keys.insert(Key::KEY_W);
        keys.insert(Key::KEY_E);
        keys.insert(Key::KEY_R);
        keys.insert(Key::KEY_T);
        keys.insert(Key::KEY_Y);
        keys.insert(Key::KEY_U);
        keys.insert(Key::KEY_I);
        keys.insert(Key::KEY_O);
        keys.insert(Key::KEY_P);
        keys.insert(Key::KEY_LEFTBRACE);
        keys.insert(Key::KEY_RIGHTBRACE);
        keys.insert(Key::KEY_ENTER);
        keys.insert(Key::KEY_LEFTCTRL);
        keys.insert(Key::KEY_A);
        keys.insert(Key::KEY_S);
        keys.insert(Key::KEY_D);
        keys.insert(Key::KEY_F);
        keys.insert(Key::KEY_G);
        keys.insert(Key::KEY_H);
        keys.insert(Key::KEY_J);
        keys.insert(Key::KEY_K);
        keys.insert(Key::KEY_L);
        keys.insert(Key::KEY_SEMICOLON);
        keys.insert(Key::KEY_APOSTROPHE);
        keys.insert(Key::KEY_GRAVE);
        keys.insert(Key::KEY_LEFTSHIFT);
        keys.insert(Key::KEY_BACKSLASH);
        keys.insert(Key::KEY_Z);
        keys.insert(Key::KEY_X);
        keys.insert(Key::KEY_C);
        keys.insert(Key::KEY_V);
        keys.insert(Key::KEY_B);
        keys.insert(Key::KEY_N);
        keys.insert(Key::KEY_M);
        keys.insert(Key::KEY_COMMA);
        keys.insert(Key::KEY_DOT);
        keys.insert(Key::KEY_SLASH);
        keys.insert(Key::KEY_RIGHTSHIFT);
        keys.insert(Key::KEY_KPASTERISK);
        keys.insert(Key::KEY_LEFTALT);
        keys.insert(Key::KEY_SPACE);
        keys.insert(Key::KEY_CAPSLOCK);
        keys.insert(Key::KEY_F1);
        keys.insert(Key::KEY_F2);
        keys.insert(Key::KEY_F3);
        keys.insert(Key::KEY_F4);
        keys.insert(Key::KEY_F5);
        keys.insert(Key::KEY_F6);
        keys.insert(Key::KEY_F7);
        keys.insert(Key::KEY_F8);
        keys.insert(Key::KEY_F9);
        keys.insert(Key::KEY_F10);
        keys.insert(Key::KEY_NUMLOCK);
        keys.insert(Key::KEY_SCROLLLOCK);
        keys.insert(Key::KEY_KP7);
        keys.insert(Key::KEY_KP8);
        keys.insert(Key::KEY_KP9);
        keys.insert(Key::KEY_KPMINUS);
        keys.insert(Key::KEY_KP4);
        keys.insert(Key::KEY_KP5);
        keys.insert(Key::KEY_KP6);
        keys.insert(Key::KEY_KPPLUS);
        keys.insert(Key::KEY_KP1);
        keys.insert(Key::KEY_KP2);
        keys.insert(Key::KEY_KP3);
        keys.insert(Key::KEY_KP0);
        keys.insert(Key::KEY_KPDOT);
        keys.insert(Key::KEY_ZENKAKUHANKAKU);
        keys.insert(Key::KEY_102ND);
        keys.insert(Key::KEY_F11);
        keys.insert(Key::KEY_F12);
        keys.insert(Key::KEY_RO);
        keys.insert(Key::KEY_KATAKANA);
        keys.insert(Key::KEY_HIRAGANA);
        keys.insert(Key::KEY_HENKAN);
        keys.insert(Key::KEY_KATAKANAHIRAGANA);
        keys.insert(Key::KEY_MUHENKAN);
        keys.insert(Key::KEY_KPJPCOMMA);
        keys.insert(Key::KEY_KPENTER);
        keys.insert(Key::KEY_RIGHTCTRL);
        keys.insert(Key::KEY_KPSLASH);
        keys.insert(Key::KEY_SYSRQ);
        keys.insert(Key::KEY_RIGHTALT);
        keys.insert(Key::KEY_LINEFEED);
        keys.insert(Key::KEY_HOME);
        keys.insert(Key::KEY_UP);
        keys.insert(Key::KEY_PAGEUP);
        keys.insert(Key::KEY_LEFT);
        keys.insert(Key::KEY_RIGHT);
        keys.insert(Key::KEY_END);
        keys.insert(Key::KEY_DOWN);
        keys.insert(Key::KEY_PAGEDOWN);
        keys.insert(Key::KEY_INSERT);
        keys.insert(Key::KEY_DELETE);
        keys.insert(Key::KEY_MACRO);
        keys.insert(Key::KEY_MUTE);
        keys.insert(Key::KEY_VOLUMEDOWN);
        keys.insert(Key::KEY_VOLUMEUP);
        keys.insert(Key::KEY_POWER);
        keys.insert(Key::KEY_KPEQUAL);
        keys.insert(Key::KEY_KPPLUSMINUS);
        keys.insert(Key::KEY_PAUSE);
        keys.insert(Key::KEY_SCALE);
        keys.insert(Key::KEY_KPCOMMA);
        keys.insert(Key::KEY_HANGEUL);
        keys.insert(Key::KEY_HANJA);
        keys.insert(Key::KEY_YEN);
        keys.insert(Key::KEY_LEFTMETA);
        keys.insert(Key::KEY_RIGHTMETA);
        keys.insert(Key::KEY_COMPOSE);
        keys.insert(Key::KEY_STOP);
        keys.insert(Key::KEY_AGAIN);
        keys.insert(Key::KEY_PROPS);
        keys.insert(Key::KEY_UNDO);
        keys.insert(Key::KEY_FRONT);
        keys.insert(Key::KEY_COPY);
        keys.insert(Key::KEY_OPEN);
        keys.insert(Key::KEY_PASTE);
        keys.insert(Key::KEY_FIND);
        keys.insert(Key::KEY_CUT);
        keys.insert(Key::KEY_HELP);
        keys.insert(Key::KEY_MENU);
        keys.insert(Key::KEY_CALC);
        keys.insert(Key::KEY_SETUP);
        keys.insert(Key::KEY_SLEEP);
        keys.insert(Key::KEY_WAKEUP);
        keys.insert(Key::KEY_FILE);
        keys.insert(Key::KEY_SENDFILE);
        keys.insert(Key::KEY_DELETEFILE);
        keys.insert(Key::KEY_XFER);
        keys.insert(Key::KEY_PROG1);
        keys.insert(Key::KEY_PROG2);
        keys.insert(Key::KEY_WWW);
        keys.insert(Key::KEY_MSDOS);
        keys.insert(Key::KEY_COFFEE);
        keys.insert(Key::KEY_DIRECTION);
        keys.insert(Key::KEY_CYCLEWINDOWS);
        keys.insert(Key::KEY_MAIL);
        keys.insert(Key::KEY_BOOKMARKS);
        keys.insert(Key::KEY_COMPUTER);
        keys.insert(Key::KEY_BACK);
        keys.insert(Key::KEY_FORWARD);
        keys.insert(Key::KEY_CLOSECD);
        keys.insert(Key::KEY_EJECTCD);
        keys.insert(Key::KEY_EJECTCLOSECD);
        keys.insert(Key::KEY_NEXTSONG);
        keys.insert(Key::KEY_PLAYPAUSE);
        keys.insert(Key::KEY_PREVIOUSSONG);
        keys.insert(Key::KEY_STOPCD);
        keys.insert(Key::KEY_RECORD);
        keys.insert(Key::KEY_REWIND);
        keys.insert(Key::KEY_PHONE);
        keys.insert(Key::KEY_ISO);
        keys.insert(Key::KEY_CONFIG);
        keys.insert(Key::KEY_HOMEPAGE);
        keys.insert(Key::KEY_REFRESH);
        keys.insert(Key::KEY_EXIT);
        keys.insert(Key::KEY_MOVE);
        keys.insert(Key::KEY_EDIT);
        keys.insert(Key::KEY_SCROLLUP);
        keys.insert(Key::KEY_SCROLLDOWN);
        keys.insert(Key::KEY_KPLEFTPAREN);
        keys.insert(Key::KEY_KPRIGHTPAREN);
        keys.insert(Key::KEY_NEW);
        keys.insert(Key::KEY_REDO);
        keys.insert(Key::KEY_F13);
        keys.insert(Key::KEY_F14);
        keys.insert(Key::KEY_F15);
        keys.insert(Key::KEY_F16);
        keys.insert(Key::KEY_F17);
        keys.insert(Key::KEY_F18);
        keys.insert(Key::KEY_F19);
        keys.insert(Key::KEY_F20);
        keys.insert(Key::KEY_F21);
        keys.insert(Key::KEY_F22);
        keys.insert(Key::KEY_F23);
        keys.insert(Key::KEY_F24);
        keys.insert(Key::KEY_PLAYCD);
        keys.insert(Key::KEY_PAUSECD);
        keys.insert(Key::KEY_PROG3);
        keys.insert(Key::KEY_PROG4);
        keys.insert(Key::KEY_DASHBOARD);
        keys.insert(Key::KEY_SUSPEND);
        keys.insert(Key::KEY_CLOSE);
        keys.insert(Key::KEY_PLAY);
        keys.insert(Key::KEY_FASTFORWARD);
        keys.insert(Key::KEY_BASSBOOST);
        keys.insert(Key::KEY_PRINT);
        keys.insert(Key::KEY_HP);
        keys.insert(Key::KEY_CAMERA);
        keys.insert(Key::KEY_SOUND);
        keys.insert(Key::KEY_QUESTION);
        keys.insert(Key::KEY_EMAIL);
        keys.insert(Key::KEY_CHAT);
        keys.insert(Key::KEY_SEARCH);
        keys.insert(Key::KEY_CONNECT);
        keys.insert(Key::KEY_FINANCE);
        keys.insert(Key::KEY_SPORT);
        keys.insert(Key::KEY_SHOP);
        keys.insert(Key::KEY_ALTERASE);
        keys.insert(Key::KEY_CANCEL);
        keys.insert(Key::KEY_BRIGHTNESSDOWN);
        keys.insert(Key::KEY_BRIGHTNESSUP);
        keys.insert(Key::KEY_MEDIA);
        keys.insert(Key::KEY_SWITCHVIDEOMODE);
        keys.insert(Key::KEY_KBDILLUMTOGGLE);
        keys.insert(Key::KEY_KBDILLUMDOWN);
        keys.insert(Key::KEY_KBDILLUMUP);
        keys.insert(Key::KEY_SEND);
        keys.insert(Key::KEY_REPLY);
        keys.insert(Key::KEY_FORWARDMAIL);
        keys.insert(Key::KEY_SAVE);
        keys.insert(Key::KEY_DOCUMENTS);
        keys.insert(Key::KEY_BATTERY);
        keys.insert(Key::KEY_BLUETOOTH);
        keys.insert(Key::KEY_WLAN);
        keys.insert(Key::KEY_UWB);
        keys.insert(Key::KEY_UNKNOWN);
        keys.insert(Key::KEY_VIDEO_NEXT);
        keys.insert(Key::KEY_VIDEO_PREV);
        keys.insert(Key::KEY_BRIGHTNESS_CYCLE);
        keys.insert(Key::KEY_BRIGHTNESS_AUTO);
        keys.insert(Key::KEY_DISPLAY_OFF);
        keys.insert(Key::KEY_WWAN);
        keys.insert(Key::KEY_RFKILL);
        keys.insert(Key::KEY_MICMUTE);
        keys.insert(Key::BTN_0);
        keys.insert(Key::BTN_1);
        keys.insert(Key::BTN_2);
        keys.insert(Key::BTN_3);
        keys.insert(Key::BTN_4);
        keys.insert(Key::BTN_5);
        keys.insert(Key::BTN_6);
        keys.insert(Key::BTN_7);
        keys.insert(Key::BTN_8);
        keys.insert(Key::BTN_9);
        keys.insert(Key::BTN_LEFT);
        keys.insert(Key::BTN_RIGHT);
        keys.insert(Key::BTN_MIDDLE);
        keys.insert(Key::BTN_SIDE);
        keys.insert(Key::BTN_EXTRA);
        keys.insert(Key::BTN_FORWARD);
        keys.insert(Key::BTN_BACK);
        keys.insert(Key::BTN_TASK);
        keys.insert(Key::BTN_TRIGGER);
        keys.insert(Key::BTN_THUMB);
        keys.insert(Key::BTN_THUMB2);
        keys.insert(Key::BTN_TOP);
        keys.insert(Key::BTN_TOP2);
        keys.insert(Key::BTN_PINKIE);
        keys.insert(Key::BTN_BASE);
        keys.insert(Key::BTN_BASE2);
        keys.insert(Key::BTN_BASE3);
        keys.insert(Key::BTN_BASE4);
        keys.insert(Key::BTN_BASE5);
        keys.insert(Key::BTN_BASE6);
        keys.insert(Key::BTN_DEAD);
        keys.insert(Key::BTN_SOUTH);
        keys.insert(Key::BTN_EAST);
        keys.insert(Key::BTN_C);
        keys.insert(Key::BTN_NORTH);
        keys.insert(Key::BTN_WEST);
        keys.insert(Key::BTN_Z);
        keys.insert(Key::BTN_TL);
        keys.insert(Key::BTN_TR);
        keys.insert(Key::BTN_TL2);
        keys.insert(Key::BTN_TR2);
        keys.insert(Key::BTN_SELECT);
        keys.insert(Key::BTN_START);
        keys.insert(Key::BTN_MODE);
        keys.insert(Key::BTN_THUMBL);
        keys.insert(Key::BTN_THUMBR);
        keys.insert(Key::BTN_TOOL_PEN);
        keys.insert(Key::BTN_TOOL_RUBBER);
        keys.insert(Key::BTN_TOOL_BRUSH);
        keys.insert(Key::BTN_TOOL_PENCIL);
        keys.insert(Key::BTN_TOOL_AIRBRUSH);
        keys.insert(Key::BTN_TOOL_FINGER);
        keys.insert(Key::BTN_TOOL_MOUSE);
        keys.insert(Key::BTN_TOOL_LENS);
        keys.insert(Key::BTN_TOOL_QUINTTAP);
        keys.insert(Key::BTN_TOUCH);
        keys.insert(Key::BTN_STYLUS);
        keys.insert(Key::BTN_STYLUS2);
        keys.insert(Key::BTN_TOOL_DOUBLETAP);
        keys.insert(Key::BTN_TOOL_TRIPLETAP);
        keys.insert(Key::BTN_TOOL_QUADTAP);
        keys.insert(Key::BTN_GEAR_DOWN);
        keys.insert(Key::BTN_GEAR_UP);
        keys.insert(Key::KEY_OK);
        keys.insert(Key::KEY_SELECT);
        keys.insert(Key::KEY_GOTO);
        keys.insert(Key::KEY_CLEAR);
        keys.insert(Key::KEY_POWER2);
        keys.insert(Key::KEY_OPTION);
        keys.insert(Key::KEY_INFO);
        keys.insert(Key::KEY_TIME);
        keys.insert(Key::KEY_VENDOR);
        keys.insert(Key::KEY_ARCHIVE);
        keys.insert(Key::KEY_PROGRAM);
        keys.insert(Key::KEY_CHANNEL);
        keys.insert(Key::KEY_FAVORITES);
        keys.insert(Key::KEY_EPG);
        keys.insert(Key::KEY_PVR);
        keys.insert(Key::KEY_MHP);
        keys.insert(Key::KEY_LANGUAGE);
        keys.insert(Key::KEY_TITLE);
        keys.insert(Key::KEY_SUBTITLE);
        keys.insert(Key::KEY_ANGLE);
        keys.insert(Key::KEY_ZOOM);
        keys.insert(Key::KEY_MODE);
        keys.insert(Key::KEY_KEYBOARD);
        keys.insert(Key::KEY_SCREEN);
        keys.insert(Key::KEY_PC);
        keys.insert(Key::KEY_TV);
        keys.insert(Key::KEY_TV2);
        keys.insert(Key::KEY_VCR);
        keys.insert(Key::KEY_VCR2);
        keys.insert(Key::KEY_SAT);
        keys.insert(Key::KEY_SAT2);
        keys.insert(Key::KEY_CD);
        keys.insert(Key::KEY_TAPE);
        keys.insert(Key::KEY_RADIO);
        keys.insert(Key::KEY_TUNER);
        keys.insert(Key::KEY_PLAYER);
        keys.insert(Key::KEY_TEXT);
        keys.insert(Key::KEY_DVD);
        keys.insert(Key::KEY_AUX);
        keys.insert(Key::KEY_MP3);
        keys.insert(Key::KEY_AUDIO);
        keys.insert(Key::KEY_VIDEO);
        keys.insert(Key::KEY_DIRECTORY);
        keys.insert(Key::KEY_LIST);
        keys.insert(Key::KEY_MEMO);
        keys.insert(Key::KEY_CALENDAR);
        keys.insert(Key::KEY_RED);
        keys.insert(Key::KEY_GREEN);
        keys.insert(Key::KEY_YELLOW);
        keys.insert(Key::KEY_BLUE);
        keys.insert(Key::KEY_CHANNELUP);
        keys.insert(Key::KEY_CHANNELDOWN);
        keys.insert(Key::KEY_FIRST);
        keys.insert(Key::KEY_LAST);
        keys.insert(Key::KEY_AB);
        keys.insert(Key::KEY_NEXT);
        keys.insert(Key::KEY_RESTART);
        keys.insert(Key::KEY_SLOW);
        keys.insert(Key::KEY_SHUFFLE);
        keys.insert(Key::KEY_BREAK);
        keys.insert(Key::KEY_PREVIOUS);
        keys.insert(Key::KEY_DIGITS);
        keys.insert(Key::KEY_TEEN);
        keys.insert(Key::KEY_TWEN);
        keys.insert(Key::KEY_VIDEOPHONE);
        keys.insert(Key::KEY_GAMES);
        keys.insert(Key::KEY_ZOOMIN);
        keys.insert(Key::KEY_ZOOMOUT);
        keys.insert(Key::KEY_ZOOMRESET);
        keys.insert(Key::KEY_WORDPROCESSOR);
        keys.insert(Key::KEY_EDITOR);
        keys.insert(Key::KEY_SPREADSHEET);
        keys.insert(Key::KEY_GRAPHICSEDITOR);
        keys.insert(Key::KEY_PRESENTATION);
        keys.insert(Key::KEY_DATABASE);
        keys.insert(Key::KEY_NEWS);
        keys.insert(Key::KEY_VOICEMAIL);
        keys.insert(Key::KEY_ADDRESSBOOK);
        keys.insert(Key::KEY_MESSENGER);
        keys.insert(Key::KEY_DISPLAYTOGGLE);
        keys.insert(Key::KEY_SPELLCHECK);
        keys.insert(Key::KEY_LOGOFF);
        keys.insert(Key::KEY_DOLLAR);
        keys.insert(Key::KEY_EURO);
        keys.insert(Key::KEY_FRAMEBACK);
        keys.insert(Key::KEY_FRAMEFORWARD);
        keys.insert(Key::KEY_CONTEXT_MENU);
        keys.insert(Key::KEY_MEDIA_REPEAT);
        keys.insert(Key::KEY_10CHANNELSUP);
        keys.insert(Key::KEY_10CHANNELSDOWN);
        keys.insert(Key::KEY_IMAGES);
        keys.insert(Key::KEY_DEL_EOL);
        keys.insert(Key::KEY_DEL_EOS);
        keys.insert(Key::KEY_INS_LINE);
        keys.insert(Key::KEY_DEL_LINE);
        keys.insert(Key::KEY_FN);
        keys.insert(Key::KEY_FN_ESC);
        keys.insert(Key::KEY_FN_F1);
        keys.insert(Key::KEY_FN_F2);
        keys.insert(Key::KEY_FN_F3);
        keys.insert(Key::KEY_FN_F4);
        keys.insert(Key::KEY_FN_F5);
        keys.insert(Key::KEY_FN_F6);
        keys.insert(Key::KEY_FN_F7);
        keys.insert(Key::KEY_FN_F8);
        keys.insert(Key::KEY_FN_F9);
        keys.insert(Key::KEY_FN_F10);
        keys.insert(Key::KEY_FN_F11);
        keys.insert(Key::KEY_FN_F12);
        keys.insert(Key::KEY_FN_1);
        keys.insert(Key::KEY_FN_2);
        keys.insert(Key::KEY_FN_D);
        keys.insert(Key::KEY_FN_E);
        keys.insert(Key::KEY_FN_F);
        keys.insert(Key::KEY_FN_S);
        keys.insert(Key::KEY_FN_B);
        keys.insert(Key::KEY_BRL_DOT1);
        keys.insert(Key::KEY_BRL_DOT2);
        keys.insert(Key::KEY_BRL_DOT3);
        keys.insert(Key::KEY_BRL_DOT4);
        keys.insert(Key::KEY_BRL_DOT5);
        keys.insert(Key::KEY_BRL_DOT6);
        keys.insert(Key::KEY_BRL_DOT7);
        keys.insert(Key::KEY_BRL_DOT8);
        keys.insert(Key::KEY_BRL_DOT9);
        keys.insert(Key::KEY_BRL_DOT10);
        keys.insert(Key::KEY_NUMERIC_0);
        keys.insert(Key::KEY_NUMERIC_1);
        keys.insert(Key::KEY_NUMERIC_2);
        keys.insert(Key::KEY_NUMERIC_3);
        keys.insert(Key::KEY_NUMERIC_4);
        keys.insert(Key::KEY_NUMERIC_5);
        keys.insert(Key::KEY_NUMERIC_6);
        keys.insert(Key::KEY_NUMERIC_7);
        keys.insert(Key::KEY_NUMERIC_8);
        keys.insert(Key::KEY_NUMERIC_9);
        keys.insert(Key::KEY_NUMERIC_STAR);
        keys.insert(Key::KEY_NUMERIC_POUND);
        keys.insert(Key::KEY_CAMERA_FOCUS);
        keys.insert(Key::KEY_WPS_BUTTON);
        keys.insert(Key::KEY_TOUCHPAD_TOGGLE);
        keys.insert(Key::KEY_TOUCHPAD_ON);
        keys.insert(Key::KEY_TOUCHPAD_OFF);
        keys.insert(Key::KEY_CAMERA_ZOOMIN);
        keys.insert(Key::KEY_CAMERA_ZOOMOUT);
        keys.insert(Key::KEY_CAMERA_UP);
        keys.insert(Key::KEY_CAMERA_DOWN);
        keys.insert(Key::KEY_CAMERA_LEFT);
        keys.insert(Key::KEY_CAMERA_RIGHT);
        keys.insert(Key::KEY_ATTENDANT_ON);
        keys.insert(Key::KEY_ATTENDANT_OFF);
        keys.insert(Key::KEY_ATTENDANT_TOGGLE);
        keys.insert(Key::KEY_LIGHTS_TOGGLE);
        keys.insert(Key::BTN_DPAD_UP);
        keys.insert(Key::BTN_DPAD_DOWN);
        keys.insert(Key::BTN_DPAD_LEFT);
        keys.insert(Key::BTN_DPAD_RIGHT);
        keys.insert(Key::KEY_ALS_TOGGLE);
        keys.insert(Key::KEY_BUTTONCONFIG);
        keys.insert(Key::KEY_TASKMANAGER);
        keys.insert(Key::KEY_JOURNAL);
        keys.insert(Key::KEY_CONTROLPANEL);
        keys.insert(Key::KEY_APPSELECT);
        keys.insert(Key::KEY_SCREENSAVER);
        keys.insert(Key::KEY_VOICECOMMAND);
        keys.insert(Key::KEY_BRIGHTNESS_MIN);
        keys.insert(Key::KEY_BRIGHTNESS_MAX);
        keys.insert(Key::KEY_KBDINPUTASSIST_PREV);
        keys.insert(Key::KEY_KBDINPUTASSIST_NEXT);
        keys.insert(Key::KEY_KBDINPUTASSIST_PREVGROUP);
        keys.insert(Key::KEY_KBDINPUTASSIST_NEXTGROUP);
        keys.insert(Key::KEY_KBDINPUTASSIST_ACCEPT);
        keys.insert(Key::KEY_KBDINPUTASSIST_CANCEL);
        keys.insert(Key::BTN_TRIGGER_HAPPY1);
        keys.insert(Key::BTN_TRIGGER_HAPPY2);
        keys.insert(Key::BTN_TRIGGER_HAPPY3);
        keys.insert(Key::BTN_TRIGGER_HAPPY4);
        keys.insert(Key::BTN_TRIGGER_HAPPY5);
        keys.insert(Key::BTN_TRIGGER_HAPPY6);
        keys.insert(Key::BTN_TRIGGER_HAPPY7);
        keys.insert(Key::BTN_TRIGGER_HAPPY8);
        keys.insert(Key::BTN_TRIGGER_HAPPY9);
        keys.insert(Key::BTN_TRIGGER_HAPPY10);
        keys.insert(Key::BTN_TRIGGER_HAPPY11);
        keys.insert(Key::BTN_TRIGGER_HAPPY12);
        keys.insert(Key::BTN_TRIGGER_HAPPY13);
        keys.insert(Key::BTN_TRIGGER_HAPPY14);
        keys.insert(Key::BTN_TRIGGER_HAPPY15);
        keys.insert(Key::BTN_TRIGGER_HAPPY16);
        keys.insert(Key::BTN_TRIGGER_HAPPY17);
        keys.insert(Key::BTN_TRIGGER_HAPPY18);
        keys.insert(Key::BTN_TRIGGER_HAPPY19);
        keys.insert(Key::BTN_TRIGGER_HAPPY20);
        keys.insert(Key::BTN_TRIGGER_HAPPY21);
        keys.insert(Key::BTN_TRIGGER_HAPPY22);
        keys.insert(Key::BTN_TRIGGER_HAPPY23);
        keys.insert(Key::BTN_TRIGGER_HAPPY24);
        keys.insert(Key::BTN_TRIGGER_HAPPY25);
        keys.insert(Key::BTN_TRIGGER_HAPPY26);
        keys.insert(Key::BTN_TRIGGER_HAPPY27);
        keys.insert(Key::BTN_TRIGGER_HAPPY28);
        keys.insert(Key::BTN_TRIGGER_HAPPY29);
        keys.insert(Key::BTN_TRIGGER_HAPPY30);
        keys.insert(Key::BTN_TRIGGER_HAPPY31);
        keys.insert(Key::BTN_TRIGGER_HAPPY32);
        keys.insert(Key::BTN_TRIGGER_HAPPY33);
        keys.insert(Key::BTN_TRIGGER_HAPPY34);
        keys.insert(Key::BTN_TRIGGER_HAPPY35);
        keys.insert(Key::BTN_TRIGGER_HAPPY36);
        keys.insert(Key::BTN_TRIGGER_HAPPY37);
        keys.insert(Key::BTN_TRIGGER_HAPPY38);
        keys.insert(Key::BTN_TRIGGER_HAPPY39);
        keys.insert(Key::BTN_TRIGGER_HAPPY40);
    }

    let mut rel_axes = AttributeSet::<RelativeAxisType>::new();
    {
        rel_axes.insert(RelativeAxisType::REL_X);
        rel_axes.insert(RelativeAxisType::REL_Y);
        rel_axes.insert(RelativeAxisType::REL_Z);
        rel_axes.insert(RelativeAxisType::REL_RX);
        rel_axes.insert(RelativeAxisType::REL_RY);
        rel_axes.insert(RelativeAxisType::REL_RZ);
        rel_axes.insert(RelativeAxisType::REL_HWHEEL);
        rel_axes.insert(RelativeAxisType::REL_DIAL);
        rel_axes.insert(RelativeAxisType::REL_WHEEL);
        rel_axes.insert(RelativeAxisType::REL_MISC);
        rel_axes.insert(RelativeAxisType::REL_RESERVED);
        rel_axes.insert(RelativeAxisType::REL_WHEEL_HI_RES);
        rel_axes.insert(RelativeAxisType::REL_HWHEEL_HI_RES);
    }

    let device = VirtualDeviceBuilder::new()?
        .name("NHK")
        .with_keys(&keys)?
        .with_relative_axes(&rel_axes)?
        .build()
        .unwrap();

    return Ok((
        dev,
        device,
    ));
}

fn next_event(dev: &mut Device) -> Result<InInputEvent, std::io::Error> {
    return dev.next_event(ReadFlag::NORMAL | ReadFlag::BLOCKING).map(|val| val.1);
}

pub fn passthrough_ev(ev: InInputEvent, tx: &Sender<OutInputEvent>) {
    tx.send(OutInputEvent::from(ev.as_raw())).unwrap();
}

pub fn send_key(tx: &Sender<OutInputEvent>, key: Key, value: i32) {
    tx.send(OutInputEvent::new_now(EventType::KEY, key.code(), value)).unwrap();
}

pub fn send_syn(tx: &Sender<OutInputEvent>) {
    tx.send(OutInputEvent::new_now(EventType::SYNCHRONIZATION, 0, 0)).unwrap();
}

pub fn sleep(duration: u64)  {
    thread::sleep(time::Duration::from_millis(duration));
}

fn read_loop(dev: &mut Device, tx: Sender<OutInputEvent>, event_handler: EventHandler) {
    dev.grab(GrabMode::Grab).unwrap();

    loop {
        let ev = next_event(dev);
        match ev {
            Ok(ev) => if event_handler(ev, &tx) { break; },
            Err(_e) => (),
        }
    }
}

pub fn run(dev_path: String, event_handler: EventHandler) {
    let debug = match env::var("DEBUG") {
        Ok(val) => val == "1",
        Err(_) => false,
    };

    let (mut dev, mut uinput) = dev_uinput_from_file(dev_path).unwrap();

    let (tx, rx): (Sender<OutInputEvent>, Receiver<OutInputEvent>) = mpsc::channel();

    let write_loop_thread = thread::spawn(move || {
        while let Ok(ev) = rx.recv() {
            if debug { println!("{:?}", ev); }
            
            uinput.emit(&[ev]).unwrap();
        }
    });

    read_loop(&mut dev, tx, event_handler);

    write_loop_thread.join().expect("panic!");
}
