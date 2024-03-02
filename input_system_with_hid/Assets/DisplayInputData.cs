using System.Text;
using TMPro;
using UnityEngine;

public class DisplayInputData : MonoBehaviour
{
    [SerializeField] private TextMeshProUGUI displayText;
    private StringBuilder _cachedString;

    private InGameAction _inGameAction;

    private void Awake()
    {
        _inGameAction = new InGameAction();
        _cachedString = new StringBuilder(64);
    }

    private void OnEnable()
    {
        _inGameAction.Enable();
    }

    private void OnDisable()
    {
        _inGameAction.Disable();
    }

    private void Update()
    {
        var acceleration = _inGameAction.InGame.Move.ReadValue<Vector3>();
        Debug.Log(acceleration);
        _cachedString.Append("x: ");
        _cachedString.Append(acceleration.x);
        _cachedString.Append("\ny: ");
        _cachedString.Append(acceleration.y);
        _cachedString.Append("\nz: ");
        _cachedString.Append(acceleration.z);
        displayText.text = _cachedString.ToString();
        _cachedString.Clear();
    }
}
