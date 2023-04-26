<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>9. PPKD - Inatura Provisi Dibagikan</name>
   <tag></tag>
   <elementGuidId>df014f1f-a14a-49f3-bbe1-15354d225816</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;NoKontrak\&quot;: \&quot;${accno}\&quot;,\n  \&quot;IDPenerima\&quot;: ${id_penerima}, // diambil dari Detail daftar penerima\n  \&quot;AccountNo\&quot;: \&quot;${norekening}\&quot;, // diambil dari Detail daftar penerima\n  \&quot;Persentase\&quot;: ${persentase},\n  \&quot;NominalKomisi\&quot;: ${amnt_komisi}\n}\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>a230251b-ae0b-4d57-9fce-2c0c04499730</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic QkFTSUNfQVVUSDpNb2JpbGUxMjMr</value>
      <webElementGuid>ef5501f8-65f1-4626-aff8-96ab97e9da96</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://192.168.1.254:8089/api/PermohonanPembayaranKomisiDealer/InaturaProvisi/Dibagikan/NominalKomisi</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'9530708518-001'</defaultValue>
      <description></description>
      <id>ff8586d5-62d3-4b32-b195-7c9f30342aee</id>
      <masked>false</masked>
      <name>accno</name>
   </variables>
   <variables>
      <defaultValue>'1003'</defaultValue>
      <description></description>
      <id>24154d87-893c-45b2-8fb0-08d103c2ee96</id>
      <masked>false</masked>
      <name>id_penerima</name>
   </variables>
   <variables>
      <defaultValue>'1340420175'</defaultValue>
      <description></description>
      <id>2d6921e3-8dd7-4f93-a7bd-d481a44e6c8b</id>
      <masked>false</masked>
      <name>no_rekening</name>
   </variables>
   <variables>
      <defaultValue>4.0</defaultValue>
      <description></description>
      <id>a055fbe2-a314-48b9-b681-ef6123c73917</id>
      <masked>false</masked>
      <name>persentase</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>5f8cc6dd-0964-4f36-970c-25a79da50eab</id>
      <masked>false</masked>
      <name>amnt_komisi</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
