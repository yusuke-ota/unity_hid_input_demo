using System.Runtime.InteropServices;
using UnityEditor;
using UnityEngine;
using UnityEngine.InputSystem;
using UnityEngine.InputSystem.Controls;
using UnityEngine.InputSystem.Layouts;
using UnityEngine.InputSystem.LowLevel;
using UnityEngine.InputSystem.Utilities;

namespace HIDConnector
{
    [InputControlLayout(displayName = "Fake Company WioTerminal Gamepad", stateType = typeof(CustomHidState))]
    #if UNITY_EDITOR
    [InitializeOnLoad]
    #endif
    public class CustomHid : Joystick
    {
        public ButtonControl Button0 { get; private set; }
        public ButtonControl Button1 { get; private set; }
        public ButtonControl Button2 { get; private set; }
        public ButtonControl Button3 { get; private set; }
        public ButtonControl Button4 { get; private set; }
        public ButtonControl Button5 { get; private set; }
        public ButtonControl Button6 { get; private set; }
        public ButtonControl Button7 { get; private set; }
        public StickControl Stick { get; private set; }
        public IntegerControl X { get; private set; }
        public IntegerControl Y { get; private set; }
        public IntegerControl Z { get; private set; }

        // Register the device.
        static CustomHid()
        {
            InputSystem.RegisterLayout<CustomHid>(
                matches: new InputDeviceMatcher()
                    .WithInterface("HID")
                    .WithCapability("PID", 1209)
                    .WithCapability("VID", 0001));
        }
        [RuntimeInitializeOnLoadMethod(RuntimeInitializeLoadType.BeforeSceneLoad)]
        private static void InitializeInPlayer() {}

        protected override void FinishSetup()
        {
            base.FinishSetup();
            Button0 = GetChildControl<ButtonControl>("button0");
            Button1 = GetChildControl<ButtonControl>("button1");
            Button2 = GetChildControl<ButtonControl>("button2");
            Button3 = GetChildControl<ButtonControl>("button3");
            Button4 = GetChildControl<ButtonControl>("button4");
            Button5 = GetChildControl<ButtonControl>("button5");
            Button6 = GetChildControl<ButtonControl>("button6");
            Button7 = GetChildControl<ButtonControl>("button7");
            X = GetChildControl<IntegerControl>("x");
            Y = GetChildControl<IntegerControl>("y");
            Z = GetChildControl<IntegerControl>("z");
        }
    }

    [StructLayout(LayoutKind.Explicit, Size = 64)]
    public struct CustomHidState : IInputStateTypeInfo
    {
        public FourCC format => new FourCC('H', 'I', 'D');

        [FieldOffset(0)]
        [InputControl(name = "button0", layout = "Button", bit = 0)]
        [InputControl(name = "button1", layout = "Button", bit = 1)]
        [InputControl(name = "button2", layout = "Button", bit = 2)]
        [InputControl(name = "button3", layout = "Button", bit = 3)]
        [InputControl(name = "button4", layout = "Button", bit = 4)]
        [InputControl(name = "button5", layout = "Button", bit = 5)]
        [InputControl(name = "button6", layout = "Button", bit = 6)]
        [InputControl(name = "button7", layout = "Button", bit = 7)]
        public ushort buttons;

        [InputControl(name = "x", layout = "Integer")] [FieldOffset(1)]
        public short x;
        
        [InputControl(name = "y", layout = "Integer")] [FieldOffset(2)]
        public short y;
        
        [InputControl(name = "z", layout = "Integer")] [FieldOffset(3)]
        public short z;
    }
}
