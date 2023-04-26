import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import groovy.json.JsonSlurper

def text = """
{
    "DaftarPenerima": [
        {
            "IdPenerima": 41917,
            "NamaPenerima": "PT ASIA BERJAYA MOBILINDO",
            "JabatanPenerima": "CORPORATE",
            "TipePenerima": "Corporate",
            "Alamat": "JL BRIGJEN DHARSONO NO 8 BLOK G-H",
            "RecordStatus": "Active"
        },
        {
            "IdPenerima": 41989,
            "NamaPenerima": "VENDOR",
            "JabatanPenerima": "CORPORATE",
            "TipePenerima": "Corporate",
            "Alamat": "-",
            "RecordStatus": "Active"
        },
        {
            "IdPenerima": 4025,
            "NamaPenerima": "MULYANA MS MARJUKI",
            "JabatanPenerima": "BM",
            "TipePenerima": "Individual",
            "Alamat": "KATIASA BARU KEL KALIJAGA KEC HARJAMUKTI ",
            "RecordStatus": "Active"
        },
        {
            "IdPenerima": 4412,
            "NamaPenerima": "IRAWAN NURISMAN",
            "JabatanPenerima": "Pengelola",
            "TipePenerima": "Individual",
            "Alamat": "JL. CEMARA I NO 6 KOMPLEK AI. RT.004 RW.003, PANGKALAN JATI, CINERE, KOTA DEPOK",
            "RecordStatus": "Active"
        },
        {
            "IdPenerima": 7566,
            "NamaPenerima": "ACHMAD ERWINSYAH",
            "JabatanPenerima": "BM",
            "TipePenerima": "Individual",
            "Alamat": "-",
            "RecordStatus": "Active"
        },
        {
            "IdPenerima": 14610,
            "NamaPenerima": "TOGA LAMHOT SINAGA",
            "JabatanPenerima": "OM",
            "TipePenerima": "Individual",
            "Alamat": "-",
            "RecordStatus": "Active"
        },
        {
            "IdPenerima": 15385,
            "NamaPenerima": "INDRA SANJAYA MALIN",
            "JabatanPenerima": "SALES",
            "TipePenerima": "Individual",
            "Alamat": "-",
            "RecordStatus": "Active"
        },
        {
            "IdPenerima": 23617,
            "NamaPenerima": "MURNIAWATI",
            "JabatanPenerima": "SPV / SALES HEAD / KOORDINATOR SALES",
            "TipePenerima": "Individual",
            "Alamat": "BLOK PAHING CIAWIGAJAH",
            "RecordStatus": "Active"
        },
        {
            "IdPenerima": 26731,
            "NamaPenerima": "HERRY RAHARDIAN",
            "JabatanPenerima": "SALES",
            "TipePenerima": "Individual",
            "Alamat": "KOTA CIREBON",
            "RecordStatus": "Active"
        },
        {
            "IdPenerima": 34008,
            "NamaPenerima": "RUDY",
            "JabatanPenerima": "SALES",
            "TipePenerima": "Individual",
            "Alamat": "BLOK CIKONDE RT.003 RW.002",
            "RecordStatus": "Active"
        },
        {
            "IdPenerima": 36146,
            "NamaPenerima": "GIAN WAHYU PRATAMA",
            "JabatanPenerima": "SALES",
            "TipePenerima": "Individual",
            "Alamat": "-",
            "RecordStatus": "Active"
        },
        {
            "IdPenerima": 41808,
            "NamaPenerima": "DEDDY PRINADI MUNADJI",
            "JabatanPenerima": "BM",
            "TipePenerima": "Individual",
            "Alamat": "GSP PERMAI RT/RW 005/017",
            "RecordStatus": "Active"
        },
        {
            "IdPenerima": 55026,
            "NamaPenerima": "DENISE YOESMITA",
            "JabatanPenerima": "SALES",
            "TipePenerima": "Individual",
            "Alamat": "HBTB SUKATANI B4 NO 10 RT 03 RW 017",
            "RecordStatus": "Active"
        },
        {
            "IdPenerima": 62527,
            "NamaPenerima": "VIRDA NOVIANA",
            "JabatanPenerima": "SALES",
            "TipePenerima": "Individual",
            "Alamat": "PULASAREN PEKALIPAN",
            "RecordStatus": "Active"
        }
    ],
    "TotalData": 14,
    "page": 1,
    "size": 67
}
"""

def slurper = new JsonSlurper()

def result = slurper.parseText(text)

def newf = result.DaftarPenerima.findAll { Map map ->
	map.get("NamaPenerima").contains("IRAWAN NURISMAN") }
//     ^^^^^^^^^^^^^^^^^

val = newf.JabatanPenerima

println val
