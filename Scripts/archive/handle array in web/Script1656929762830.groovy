import org.openqa.selenium.By as By
import org.openqa.selenium.WebDriver as WebDriver
import org.openqa.selenium.WebElement as WebElement
import com.kms.katalon.core.webui.driver.DriverFactory as DriverFactory
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import internal.GlobalVariable as GlobalVariable

/**
 * reproducing the Example.1 in
 * https://www.katalon.com/resources-center/tutorials/handle-web-tables/
 *
 * using arrays rather than List
 */
WebUI.openBrowser('')

WebUI.setViewPortSize(703, 347)

WebUI.navigateToUrl('http://demoaut-mimic.kazurayam.com/18107_testbed.html')

WebUI.delay(5)

WebDriver driver = DriverFactory.getWebDriver()

// To locate table
WebElement table = driver.findElement(By.xpath('//table/tbody'))

// To locate rows of table it will capture all the rows available in the table
WebElement[] rowsInTable = ((table.findElements(By.tagName('tr'))) as WebElement[])

// To calculate number of rows in the table
int rowsCount = rowsInTable.length

WebUI.comment("rowsCount=$rowsCount")

// Iterate over all the rows of the table
for (int r = 0; r < rowsCount; r++) {
    // To locate columns(cells) of that specific row
    WebElement[] columnsInRow = (((rowsInTable[r]).findElements(By.tagName('td'))) as WebElement[])

    // To calculate number of columns in that specific row
    int columnsCount = columnsInRow.length

    WebUI.comment("Number of cells in row $r are $columnsCount")

    // Iterate over the cells of that specifc row
    for (int c = 0; c < columnsCount; c++) {
        // Retrieve text from each cell
        String cellText = (columnsInRow[c]).getText()

        WebUI.comment("Cell value of ($r,$c) is '$cellText'")
    }
}

