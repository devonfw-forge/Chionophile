namespace Devon4Net.Infrastructure.CyberArk.Dto.Safe
{
    public class AddSafeResult
    {
        public string Description { get; set; }
        public string ManagingCPM { get; set; }
        public object NumberOfDaysRetention { get; set; }
        public int NumberOfVersionsRetention { get; set; }
        public bool OLACEnabled { get; set; }
        public string SafeName { get; set; }
    }
}