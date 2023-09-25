rule CRDE_binaries
{
    meta:
        author = "g0h4n_0 <g0h4n_0@protonmail.com>"
        date_created = "2023-09-25"
        date_last_modified = "2023-09-25"
        description = "Detects CRDE binaries used for RDE1 project to exfiltrate files from DNS and HTTPS queries."
        reference = "https://github.com/g0h4n/RDE1"
        hash1 = ""

    strings:
        $a1 = "crde::utils::checker"
        $a2 = "CRDE"

        $b1 = "crde::methods::dns"

        $c1 = "crde::methods::https"

    condition:
        all of ($a*) and (
        	$b1
        	or 
        	$c1
        )
}
